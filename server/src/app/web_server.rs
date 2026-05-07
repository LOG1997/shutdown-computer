use crate::{
    api::handle::{get_device_status, get_system_info, reboot_handler, shutdown_handler},
    common::{app::get_app_dir, config::WebServerConfig},
};
use axum::{Router, routing::post};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::services::ServeDir; // 用于设置 Content-Type

pub async fn start_web_server(web_server_config: &WebServerConfig) {
    let is_dev = cfg!(debug_assertions);
    let app_dir = get_app_dir();

    let static_files_root = if is_dev {
        "../client/apps/web/dist".into()
    } else {
        app_dir.join("web")
    };
    // dev的模式下检查dist目录，否则检查web目录，不存在直接报错
    if !static_files_root.exists() {
        panic!("static files not found");
    }
    // 1. 加载自签名证书
    let cert_path = app_dir.join("cert.pem");
    let key_path = app_dir.join("key.pem"); // 生产环境私钥路径

    // 3. 加载自签名证书
    let tls_config =
        RustlsConfig::from_pem_file(cert_path.to_str().unwrap(), key_path.to_str().unwrap())
            .await
            .unwrap();

    // 如果访问 / (根路径)，ServeDir 默认会尝试查找 index.html (取决于配置，通常需确保存在)
    let static_files_service =
        ServeDir::new(&static_files_root).append_index_html_on_directories(true); // 关键：访问目录时自动返回 index.html
    // 路由配置
    let app = Router::new()
        .route("/shutdown", post(shutdown_handler))
        .route("/reboot", post(reboot_handler))
        .route("/getStatus", post(get_device_status))
        .route("/getDeviceInfo", post(get_system_info))
        .fallback_service(static_files_service)
        // 允许跨域 → 网页必须
        .layer(tower_http::cors::CorsLayer::permissive());
    // 启动服务
    if web_server_config.https {
        let ip: std::net::IpAddr = web_server_config.host.parse().expect("Invalid host IP");
        let addr = std::net::SocketAddr::from((ip, web_server_config.port));
        println!("启动服务: https://{}", addr);
        axum_server::bind_rustls(addr, tls_config)
            .serve(app.into_make_service())
            .await
            .unwrap();
    } else {
        let ip: std::net::IpAddr = web_server_config.host.parse().expect("Invalid host IP");
        let addr = std::net::SocketAddr::from((ip, web_server_config.port));
        println!("启动服务: http://{}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, app).await.unwrap();
    }
}
