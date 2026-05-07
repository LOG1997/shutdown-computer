mod common;
use common::app::get_app_dir;
use common::config::AppConfig;

mod api;
mod system;

mod app;
use app::mqtt_ha::start_mqtt;
use app::web_server::start_web_server;
// 关机主函数
#[tokio::main]
async fn main() {
    let app_dir = get_app_dir();
    let app_config = AppConfig::from_file(AppConfig::default_path(&app_dir).to_str().unwrap())
        .expect("Failed to load config file");

    // 获取web服务配置
    let web_server_config = app_config.get_server();
    let enable_web_server = web_server_config.enable;

    // 获取mqtt配置
    let mqtt_config = app_config.get_mqtt();
    let enable_mqtt = mqtt_config.enable;
    if enable_web_server {
        println!("启动web服务...");
        start_web_server(web_server_config).await;
    }
    if enable_mqtt {
        println!("启动MQTT服务...");
        start_mqtt(mqtt_config).await;
    }
}
