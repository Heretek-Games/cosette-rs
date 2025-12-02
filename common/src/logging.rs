use tracing::Level;
pub fn init_tracing(level: Level) {
    eprintln!(
        r#"   ______                __  __
       / ____/___  ________  / /_/ /____        __________
      / /   / __ \/ ___/ _ \/ __/ __/ _ \______/ ___/ ___/
     / /___/ /_/ (__  )  __/ /_/ /_/  __/_____/ /  (__  )
     \____/\____/____/\___/\__/\__/\___/     /_/  /____/
                                                          "#
    );
    tracing_subscriber::fmt().with_max_level(level).init()
}
