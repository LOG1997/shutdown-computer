use axum::{Json, Router, http::StatusCode, routing::post};
use serde::Deserialize;
use std::process::Command;

// 配置
const SHUTDOWN_KEY: &str = "SHUTDOWN_NOW_123456";
const PORT: u16 = 9527;

// 接收前端传的密钥
#[derive(Deserialize)]
struct ShutdownRequest {
    key: String,
}

// 关机主函数
#[tokio::main]
async fn main() {
    println!("✅ 远程关机服务已启动");
    println!("📡 端口: {}", PORT);
    println!("🔑 密钥: {}", SHUTDOWN_KEY);

    // 路由配置
    let app = Router::new()
        .route("/shutdown", post(shutdown_handler))
        // 允许跨域 → 网页必须
        .layer(tower_http::cors::CorsLayer::permissive());

    // 启动服务
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], PORT));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 处理关机请求
async fn shutdown_handler(
    Json(body): Json<ShutdownRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    if body.key == SHUTDOWN_KEY {
        // 执行关机
        execute_shutdown();

        return (
            StatusCode::OK,
            Json(serde_json::json!({
                "code": 0,
                "msg": "关机指令已执行"
            })),
        );
    }

    (
        StatusCode::FORBIDDEN,
        Json(serde_json::json!({
            "code": -1,
            "msg": "密钥错误"
        })),
    )
}

// 跨平台关机
fn execute_shutdown() {
    println!("正在执行关机指令...");

    #[cfg(target_os = "windows")]
    {
        // Windows: shutdown /s /t 0 (立即关机)
        // 注意：Windows下通常不需要 sudo，但需要管理员权限运行此 Rust 程序
        match Command::new("shutdown")
            .arg("/s")
            .arg("/t")
            .arg("60")
            .output()
        {
            Ok(output) => {
                if output.status.success() {
                    println!("Windows 关机指令发送成功");
                } else {
                    eprintln!(
                        "Windows 关机失败: {:?}",
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }
            Err(e) => eprintln!("执行 Windows 关机命令出错: {}", e),
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Linux 尝试多种方案

        // 方案 1: 使用 systemctl (现代大多数 Linux 发行版推荐，且如果服务以 root 运行则无需 sudo)
        // 尝试直接执行 shutdown (如果程序以 root 运行)
        let status = Command::new("shutdown")
            .arg("-h")
            .arg("+1") // +1 表示 1 分钟后关机
            .status();

        match status {
            Ok(exit_status) => {
                if exit_status.success() {
                    println!("Linux systemctl poweroff 执行成功");
                } else {
                    eprintln!("systemctl poweroff 失败, 退出码: {:?}", exit_status.code());
                    // 如果失败，尝试传统 shutdown
                    fallback_shutdown_linux();
                }
            }
            Err(e) => {
                eprintln!("执行 systemctl 出错: {}, 尝试备用方案", e);
                // fallback_shutdown_linux();
            }
        }
    }
}

#[cfg(target_os = "linux")]
fn fallback_shutdown_linux() {
    println!("尝试备用关机命令: shutdown -h now");
    // 注意：这里去掉 sudo，假设当前运行该 Rust 程序的用户已经有 sudo 免密权限，或者程序本身以 root 运行
    // 如果必须用 sudo，请确保配置了 /etc/sudoers 允许该用户无密码执行 shutdown
    match Command::new("sudo")
        .arg("shutdown")
        .arg("-h")
        .arg("+1")
        .status()
    {
        Ok(exit_status) => {
            if !exit_status.success() {
                eprintln!("备用关机命令失败, 退出码: {:?}", exit_status.code());
            }
        }
        Err(e) => eprintln!("执行备用关机命令出错: {}", e),
    }
}
