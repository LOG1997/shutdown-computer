use std::path::PathBuf;

pub fn get_app_is_dev() -> bool {
    cfg!(debug_assertions)
}
pub fn get_app_dir() -> PathBuf {
    // current_exe() 获取当前运行的二进制文件的完整路径
    // parent() 获取其所在目录
    let is_dev = get_app_is_dev();
    match is_dev {
        true => std::env::current_dir().expect("Failed to get current working directory"),
        false => std::env::current_exe()
            .expect("Failed to get current executable path")
            .parent()
            .expect("Failed to get parent directory of executable")
            .to_path_buf(),
    }
}
