use crate::common::config::MqttConfig;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use std::time::Duration;

/**
 * @description: 连接MQTT服务器，并向其发送指令，用于向mqtt发送信息
 * @param {*} mqtt_config 传入mqtt的配置
 */
pub async fn start_mqtt(mqtt_config: &MqttConfig) {
    let mut mqtt_options = MqttOptions::new(
        "rust-mqtt-client-123", // 客户端ID（随便写，唯一即可）
        "localhost",            // 本地 MQTT 服务器地址
        1883,                   // 默认端口
    );
    mqtt_options.set_keep_alive(Duration::from_secs(5));

    // ====================== 2. 创建客户端 ======================
    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);

    // 异步线程处理 MQTT 事件（必须运行）
    tokio::spawn(async move {
        loop {
            let event = event_loop.poll().await.unwrap();
            println!("收到 MQTT 事件: {:?}", event);
        }
    });

    // 等待连接建立
    tokio::time::sleep(Duration::from_millis(100)).await;

    // ====================== 3. 订阅主题 ======================
    client
        .subscribe("test/topic", QoS::AtLeastOnce)
        .await
        .unwrap();
    println!("✅ 已订阅主题: test/topic");

    // ====================== 4. 发布消息 ======================
    let mut count = 0;
    loop {
        count += 1;
        let msg = format!("Rust 循环消息 -> {}", count);

        // 发送
        let _ = client
            .publish("test/loop", QoS::AtLeastOnce, false, msg)
            .await;

        println!("已发送：第 {} 条消息", count);

        // 间隔 1 秒
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
