use crate::common::config::MqttConfig;
use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde_json::json;
use std::sync::LazyLock;
use std::time::Duration;
// 定义主题与内容
// 通用实体信息
// 使用标准库 LazyLock (Rust 1.80+)
static COMMON_DEVICE_ENTITY: LazyLock<serde_json::Value> = LazyLock::new(|| {
    json!({
      "name": "我的电脑",
      "identifiers": ["pc_pc01"],
      "manufacturer": "我的电脑",
      "model": "my_computer_model"
    })
});

static COMPUTER_ONLINE_STATUS: LazyLock<serde_json::Value> = LazyLock::new(|| {
    json!({
        "homeassistant/binary_sensor/pc_pc01_info_status_online/config":
        {
            "name": "在线状态",
            "unique_id": "pc_pc01_status",
            "state_topic": "pc_pc01/info/status_online",
            "value_template": "{{ value_json.online }}",
            "payload_on": "online",
            "payload_off": "offline",
            "device_class": "connectivity",
            "expire_after": 30,
            "device": COMMON_DEVICE_ENTITY.clone(),
    }})
});

static COMPUTER_CPU_INFO: LazyLock<serde_json::Value> = LazyLock::new(|| {
    json!({
            "homeassistant/sensor/pc_pc01_info_cpu_brand/config":
            {
                "name": "CPU",
                "unique_id": "pc_pc01_cpu_brand",
                "state_topic": "pc_pc01/info/cpu",
                "value_template": "{{ value_json.brand }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
        },
       "homeassistant/sensor/pc_pc01_info_cpu_physical_core_count/config":{
                  "name": "核心数",
                "unique_id": "pc_pc01_cpu_physical_core_count",
                "state_topic": "pc_pc01/info/cpu",
                "value_template": "{{ value_json.physical_core_count }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
       },
            "homeassistant/sensor/pc_pc01_info_cpu_total_core_count/config":{
                  "name": "线程数",
                "unique_id": "pc_pc01_cpu_total_core_count",
                "state_topic": "pc_pc01/info/cpu",
                "value_template": "{{ value_json.total_core_count }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
       },
                  "homeassistant/sensor/pc_pc01_info_cpu_frequency/config":{
                  "name": "频率",
                "unique_id": "pc_pc01_cpu_frequency",
                "state_topic": "pc_pc01/info/cpu",
                "value_template": "{{ (value_json.frequency/ 1000) | round(2) }}",
                "unit_of_measurement": "GHz",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
       },
                  "homeassistant/sensor/pc_pc01_info_cpu_usage/config":{
                  "name": "CPU使用率",
                "unique_id": "pc_pc01_cpu_usage",
                "state_topic": "pc_pc01/info/cpu",
                "value_template": "{{ value_json.usage }}",
                "unit_of_measurement": "%",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
       }
    })
});
static COMPUTER_MEMORY_INFO: LazyLock<serde_json::Value> = LazyLock::new(|| {
    json!({
            "homeassistant/sensor/pc_pc01_info_memory_free_memory/config":
            {
                "name": "未使用内存",
                "unique_id": "pc_pc01_memory_free_memory",
                "state_topic": "pc_pc01/info/memory/free_memory",
                "value_template": "{{ value_json.free_memory }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
        },
           "homeassistant/sensor/pc_pc01_info_memory_used_memory/config":
            {
                "name": "已使用内存",
                "unique_id": "pc_pc01_memory_used_memory",
                "state_topic": "pc_pc01/info/memory/used_memory",
                "value_template": "{{ value_json.used_memory }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
        },
           "homeassistant/sensor/pc_pc01_info_memory_usage/config":
            {
                "name": "内存使用率",
                "unique_id": "pc_pc01_memory_usage",
                "state_topic": "pc_pc01/info/memory/usage",
                "value_template": "{{ value_json.usage }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
        },
    })
});

static COMPUTER_SYSTEM_INFO: LazyLock<serde_json::Value> = LazyLock::new(|| {
    json!({
            "homeassistant/sensor/pc_pc01_info_system_host_name/config":
            {
                "name": "电脑名称",
                "unique_id": "pc_pc01_system_host_name",
                "state_topic": "pc_pc01/info/system/host_name",
                "value_template": "{{ value_json.host_name }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
        },
              "homeassistant/sensor/pc_pc01_info_system_name/config":
            {
                "name": "系统名称",
                "unique_id": "pc_pc01_system_name",
                "state_topic": "pc_pc01/info/system/name",
                "value_template": "{{ value_json.name }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
        },
                      "homeassistant/sensor/pc_pc01_info_system_os_version/config":
            {
                "name": "系统版本",
                "unique_id": "pc_pc01_system_os_version",
                "state_topic": "pc_pc01/info/system/os_version",
                "value_template": "{{ value_json.os_version }}",
                // "device_class": "connectivity",
                "expire_after": 30,
                "device": COMMON_DEVICE_ENTITY.clone(),
        },
    })
});
/**
 * @description: 连接MQTT服务器，并向其发送指令，用于向mqtt发送信息
 * @param {*} mqtt_config 传入mqtt的配置
 */
pub async fn start_mqtt(mqtt_config: &MqttConfig) {
    let mut mqtt_options = MqttOptions::new(
        mqtt_config.client_id.clone(), // 客户端ID（随便写，唯一即可）
        mqtt_config.host.clone(),      // 本地 MQTT 服务器地址
        mqtt_config.port,              // 默认端口
    );
    mqtt_options.set_keep_alive(Duration::from_secs(mqtt_config.interval));

    // ====================== 2. 创建客户端 ======================
    let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);

    // 异步线程处理 MQTT 事件（必须运行）
    tokio::spawn(async move {
        loop {
            match event_loop.poll().await {
                Ok(notification) => {
                    // 只关心传入的消息事件
                    if let Event::Incoming(Packet::Publish(p)) = notification {
                        // p.topic: 消息的主题 (String)
                        // p.payload: 消息的内容 (Bytes)

                        // 将 payload 转换为字符串进行打印
                        // 注意：如果消息不是 UTF-8 编码，from_utf8 可能会失败
                        let payload_str = String::from_utf8_lossy(&p.payload);

                        println!("📩 收到消息:");
                        println!("   主题: {}", p.topic);
                        println!("   内容: {}", payload_str);
                    } else {
                        // 可选：打印其他类型的事件用于调试
                        // println!("其他事件: {:?}", notification);
                    }
                }
                Err(e) => {
                    eprintln!("MQTT 事件循环错误: {:?}", e);
                    break; // 出错时退出循环，避免无限打印错误
                }
            }
        }
    });

    // 等待连接建立
    tokio::time::sleep(Duration::from_millis(mqtt_config.expire_time)).await;

    // ====================== 3. 订阅主题 ======================
    client
        .subscribe("test/topic", QoS::AtLeastOnce)
        .await
        .unwrap();
    println!("✅ 已订阅主题: test/topic");
    // ====================== 4. 订阅主题 ======================
    // 提取配置信息，发送消息创建mqtt实体，遍历json COMPUTER_ONLINE_STATUS
    // for (topic, payload) in COMPUTER_ONLINE_STATUS.as_object().unwrap() {
    //     client
    //         .publish(topic, QoS::AtLeastOnce, false, payload.to_string())
    //         .await
    //         .unwrap();
    // }
    for (topic, payload) in COMPUTER_SYSTEM_INFO.as_object().unwrap() {
        println!("主题: {};副在：{}", topic, payload);
        client
            .publish(topic, QoS::AtLeastOnce, false, payload.to_string())
            .await
            .unwrap();
    }
    for (topic, payload) in COMPUTER_CPU_INFO.as_object().unwrap() {
        client
            .publish(topic, QoS::AtLeastOnce, false, payload.to_string())
            .await
            .unwrap();
    }
    for (topic, payload) in COMPUTER_MEMORY_INFO.as_object().unwrap() {
        client
            .publish(topic, QoS::AtLeastOnce, false, payload.to_string())
            .await
            .unwrap();
    }

    // ====================== 5. 发布消息 ======================
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
        tokio::time::sleep(Duration::from_secs(mqtt_config.interval)).await;
    }
}
