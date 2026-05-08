use std::path::Path;
use std::process::Command;

pub fn is_app_available(app_name: &str) -> bool {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("where").arg(app_name).output();
        return output.map(|o| o.status.success()).unwrap_or(false);
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    {
        let output = Command::new("which").arg(app_name).output();
        return output.map(|o| o.status.success()).unwrap_or(false);
    }
}

pub fn launch_app_safe(app_name: &str) -> Result<(), String> {
    if !is_app_available(app_name) {
        return Err(format!("应用 {} 未找到", app_name));
    }

    #[cfg(target_os = "windows")]
    {
        Command::new(app_name)
            .spawn()
            .map_err(|e| format!("启动失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-a")
            .arg(app_name)
            .spawn()
            .map_err(|e| format!("启动失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        Command::new(app_name)
            .spawn()
            .map_err(|e| format!("启动失败: {}", e))?;
    }

    Ok(())
}
