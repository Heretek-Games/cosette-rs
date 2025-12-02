use std::{collections::HashMap, env, ffi::OsStr, fs, io::Write, path::PathBuf};

fn collect_proto_paths(src: &std::path::Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(src).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            collect_proto_paths(&path, out);
        } else if path.extension().map(|e| e == "proto").unwrap_or(false) {
            out.push(path);
        }
    }
}

fn fix_optimize_for_in_text(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        if !line.contains("optimize_for") {
            out.push_str(line);
            out.push('\n');
            continue;
        }
        let mut replaced = false;
        if let Some(eq_pos) = line.find('=') {
            let after_eq = &line[eq_pos + 1..];
            let maybe_num = after_eq.trim_start();
            let mut num_digits = 0usize;
            for ch in maybe_num.chars() {
                if ch.is_ascii_digit() {
                    num_digits += 1;
                } else {
                    break;
                }
            }
            if num_digits > 0 {
                let num_str = &maybe_num[..num_digits];
                let replacement = match num_str {
                    "1" => "SPEED",
                    "2" => "CODE_SIZE",
                    "3" => "LITE_RUNTIME",
                    _ => "",
                };
                if !replacement.is_empty() {
                    let prefix = &line[..eq_pos + 1];
                    let remainder = &maybe_num[num_digits..];
                    let ws_len = after_eq.len() - maybe_num.len();
                    let ws = &after_eq[..ws_len];
                    let new_after = format!("{}{}{}", ws, replacement, remainder);
                    let new_line = format!("{}{}", prefix, new_after);
                    out.push_str(&new_line);
                    out.push('\n');
                    replaced = true;
                }
            }
        }
        if !replaced {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    let proto_root = PathBuf::from("/home/yohk4e/Downloads/proto/protos");
    if !proto_root.exists() {
        panic!("proto root not found: {}", proto_root.display());
    }

    // Collect protos
    let mut proto_paths = Vec::<PathBuf>::new();
    collect_proto_paths(&proto_root, &mut proto_paths);
    if proto_paths.is_empty() {
        println!(
            "cargo:warning=No .proto files found under {}",
            proto_root.display()
        );
        return Ok(());
    }

    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let fixed_root = out_dir.join("fixed_protos");
    if fixed_root.exists() {
        std::fs::remove_dir_all(&fixed_root)?;
    }
    std::fs::create_dir_all(&fixed_root)?;
    for src in &proto_paths {
        let rel = src.strip_prefix(&proto_root).unwrap();
        let dst = fixed_root.join(rel);
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let txt = std::fs::read_to_string(src)?;
        let fixed = fix_optimize_for_in_text(&txt);
        std::fs::write(&dst, fixed)?;
        println!("cargo:rerun-if-changed={}", src.display());
    }

    let gen_rs_dir = out_dir.join("generated_rs");
    if gen_rs_dir.exists() {
        std::fs::remove_dir_all(&gen_rs_dir)?;
    }
    std::fs::create_dir_all(&gen_rs_dir)?;

    let mut fixed_proto_strs = Vec::<String>::new();
    for entry in walkdir::WalkDir::new(&fixed_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().map(|e| e == "proto").unwrap_or(false) {
            fixed_proto_strs.push(path.canonicalize()?.to_string_lossy().into_owned());
        }
    }

    let includes_owned = fixed_root.to_string_lossy().into_owned();
    let includes = &[includes_owned.as_str()];

    let mut config = tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir(gen_rs_dir.clone())
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_well_known_types(true)
        .disable_package_emission();

    // Compile all proto files
    config.compile(&fixed_proto_strs, includes)?;

    // Copy generated .rs files into include/
    let include_dir = manifest_dir.join("include");
    if include_dir.exists() {
        std::fs::remove_dir_all(&include_dir)?;
    }
    std::fs::create_dir_all(&include_dir)?;

    let mut module_entries: Vec<(String, String)> = Vec::new();
    for entry in walkdir::WalkDir::new(&gen_rs_dir)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("rs") {
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let module_path = file_name.trim_end_matches(".rs").to_string();
            let content = fs::read_to_string(&path)?;
            module_entries.push((module_path, content));
        }
    }

    // collect a set/list of all module paths
    let module_paths: Vec<String> = module_entries.iter().map(|(p, _)| p.clone()).collect();

    for (module_path, content) in &module_entries {
        // check for nested children
        let prefix_dot = format!("{}.", module_path);
        let has_nested = module_paths
            .iter()
            .any(|p| p != module_path && p.starts_with(&prefix_dot));

        let parts: Vec<&str> = module_path.split('.').collect();

        if has_nested {
            let mut dir_path = include_dir.clone();
            for part in &parts {
                dir_path.push(part);
            }
            fs::create_dir_all(&dir_path)?;
            let dest = dir_path.join("mod.rs");
            fs::write(&dest, content)?;
        } else {
            // no nested children, write as normal file
            if parts.len() > 1 {
                let dir_parts = &parts[..parts.len() - 1];
                let file_part = parts[parts.len() - 1];
                let mut current_path = include_dir.clone();
                for part in dir_parts {
                    current_path.push(part);
                }
                fs::create_dir_all(&current_path)?;
                let dest_file = current_path.join(format!("{}.rs", file_part));
                fs::write(&dest_file, content)?;
            } else {
                let dest_file = include_dir.join(format!("{}.rs", module_path));
                fs::write(&dest_file, content)?;
            }
        }
    }

    create_mod_files_recursive(&include_dir)?;

    println!(
        "cargo:warning=Successfully generated modules in {}",
        include_dir.display()
    );
    println!("cargo:rerun-if-changed={}", proto_root.display());

    Ok(())
}

fn create_mod_files_recursive(dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Collect children and files for this directory
    let mut child_dirs: Vec<String> = Vec::new();
    let mut rs_files: Vec<String> = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = path
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap()
            .to_string();

        if path.is_dir() {
            child_dirs.push(name);
        } else if path.is_file() {
            if path.extension().and_then(OsStr::to_str) == Some("rs") {
                if name != "mod.rs" {
                    let stem = path
                        .file_stem()
                        .and_then(OsStr::to_str)
                        .unwrap()
                        .to_string();
                    rs_files.push(stem);
                }
            }
        }
    }

    child_dirs.sort();
    rs_files.sort();

    for child in &child_dirs {
        let mut child_path = dir.clone();
        child_path.push(child);
        create_mod_files_recursive(&child_path)?;
    }

    let mod_path = dir.join("mod.rs");
    let auto_marker = "// AUTO-GENERATED SUBMODULES - DO NOT EDIT";

    if mod_path.exists() {
        let existing = fs::read_to_string(&mod_path)?;
        if existing.contains(auto_marker) {
            return Ok(());
        }

        // Append an auto-generated block to list submodules.
        let mut f = fs::OpenOptions::new().append(true).open(&mod_path)?;
        writeln!(f)?;
        writeln!(f, "{}", auto_marker)?;
        writeln!(f)?;

        for child in &child_dirs {
            writeln!(f, "pub mod {};", child)?;
        }

        for file in &rs_files {
            writeln!(f, "pub mod {};", file)?;
        }
    } else {
        // Create fresh mod.rs with declarations
        let mut f = fs::File::create(&mod_path)?;
        writeln!(f, "// Auto-generated module file")?;
        writeln!(f, "// DO NOT EDIT")?;
        writeln!(f)?;
        writeln!(f, "{}", auto_marker)?;
        writeln!(f)?;

        for child in &child_dirs {
            writeln!(f, "pub mod {};", child)?;
        }

        for file in &rs_files {
            writeln!(f, "pub mod {};", file)?;
        }
    }

    Ok(())
}
