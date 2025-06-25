# 📘 MQTT Logger in Rust

A simple MQTT subscriber built with Rust that listens to the `/broker_alive` topic and stores each incoming message as a JSON record in a local SQLite database.

---

## 🚀 Features

- Connects to a public MQTT broker (`broker.hivemq.com`)
- Subscribes to the topic `/broker_alive`
- Parses and validates JSON messages
- Saves messages into a local SQLite database with:
  - Auto-incremented ID
  - Topic
  - JSON message
  - Timestamp (Unix epoch)

---

## 🧱 Tech Stack

- [`rumqttc`](https://docs.rs/rumqttc): MQTT client for Rust
- [`rusqlite`](https://docs.rs/rusqlite): SQLite database access
- [`serde`](https://serde.rs/): Serialization/deserialization
- [`tokio`](https://tokio.rs/): Async runtime

---

## 📦 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.60+)
- SQLite (optional, for CLI inspection)

### Clone and Run

```bash
git clone https://github.com/yourname/mqtt_logger_rust.git
cd mqtt_logger_rust
cargo run
```

## 📁 Project Structure
```
mqtt_logger_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

## 📝 Database Schema
The app creates a local file mqtt_logs.db with a messages table using the following schema:
``` SQL
CREATE TABLE IF NOT EXISTS messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    topic TEXT NOT NULL,
    message TEXT NOT NULL,
    timestamp INTEGER NOT NULL
);
```

## 💡 Usage
Once the application is running, it will:
Connect to the MQTT broker
Subscribe to /broker_alive
Save all incoming valid JSON messages into the mqtt_logs.db file

### Example MQTT Message

``` JSON
{
  "status": "alive",
  "service": "laundry_backend"
}
```

## 📂 Dependencies

``` toml

[dependencies]
rumqttc = "0.19"
tokio = { version = "1", features = ["full"] }
rusqlite = { version = "0.29", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1.0"
```