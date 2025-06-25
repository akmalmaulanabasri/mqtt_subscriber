use rumqttc::{MqttOptions, Client, QoS, Event, Incoming};
use rusqlite::{params, Connection};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::task;
use serde_json::Value;
use dotenvy::dotenv;
use std::env;

fn init_db() -> Connection {
    let conn = Connection::open("mqtt_logs.db").expect("Failed to open database");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            topic TEXT NOT NULL,
            message TEXT NOT NULL,
            timestamp INTEGER NOT NULL
        )",
        [],
    ).expect("Failed to create table");

    conn
}

fn save_message(conn: &Connection, topic: &str, message: &str, timestamp: i64) {
    conn.execute(
        "INSERT INTO messages (topic, message, timestamp) VALUES (?1, ?2, ?3)",
        params![topic, message, timestamp],
    ).expect("Failed to insert message");
}

#[tokio::main]
async fn main() {
    dotenv().ok(); // Load .env file

    let mqtt_host = env::var("MQTT_BROKER_HOST").unwrap_or("172.10.20.53".to_string());
    let mqtt_port: u16 = env::var("MQTT_BROKER_PORT").unwrap_or("1883".to_string()).parse().unwrap();
    let mqtt_username = env::var("MQTT_USERNAME").unwrap_or_default();
    let mqtt_password = env::var("MQTT_PASSWORD").unwrap_or_default();

    let db = init_db();
    let db_conn = db;

    let mut mqttoptions = MqttOptions::new("rust-logger", mqtt_host, mqtt_port);
    mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));
    mqttoptions.set_credentials(mqtt_username, mqtt_password);

    let (mut client, mut connection) = Client::new(mqttoptions, 10);

    client.subscribe("/broker_alive", QoS::AtMostOnce).unwrap();
    println!("✅ Subscribed to /broker_alive");

    task::spawn_blocking(move || {
        for notification in connection.iter() {
            match notification {
                Ok(Event::Incoming(Incoming::Publish(publish))) => {
                    let topic = publish.topic;
                    let payload = String::from_utf8_lossy(&publish.payload).to_string();

                    println!("📥 [{}]: {}", topic, payload);

                    let json: Value = match serde_json::from_str(&payload) {
                        Ok(j) => j,
                        Err(_) => {
                            eprintln!("❌ Invalid JSON, skipped");
                            continue;
                        }
                    };

                    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

                    save_message(&db_conn, &topic, &json.to_string(), now);
                }
                Err(e) => {
                    eprintln!("❌ MQTT Error: {:?}", e);
                    break;
                }
                _ => {}
            }
        }
    });

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}
