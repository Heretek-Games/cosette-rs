// src/lib.rs
// 主库文件

// 引入生成的 protobuf 代码
pub mod include {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/include/mod.rs"));
}

// 重新导出常用模块
pub use include::*;

// 为方便使用，创建一些常用的类型别名
pub mod prelude {
    pub use super::*;
}

// 如果需要，可以在这里添加一些辅助函数或 trait 实现
