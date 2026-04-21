// src/config.rs
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub https: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SecurityConfig {
    pub shutdown_key: String,
}

impl AppConfig {
    /// 从指定路径加载配置文件
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: AppConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// 获取默认的配置文件路径（例如在 app_dir 下）
    pub fn default_path(app_dir: &PathBuf) -> PathBuf {
        app_dir.join("config.toml")
    }
    // 获取server
    pub fn get_server(&self) -> &ServerConfig {
        &self.server
    }
    // 获取security
    pub fn get_security(&self) -> &SecurityConfig {
        &self.security
    }
}
