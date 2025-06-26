//! lib.rs 是 库 crate 的入口文件，其作用是定义和导出公共的模块、结构体、函数等供其他模块或外部程序使用。

mod opts;
mod process;

pub use opts::{Opts, SubCommand, GenPassOpts};
pub use process::*;