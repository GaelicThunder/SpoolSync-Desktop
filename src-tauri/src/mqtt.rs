use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, TlsConfiguration, Transport};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BambuPrinterConfig {
    pub name: String,
    pub ip_address: String,
    pub access_code: String,
    pub serial_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilamentSyncCommand {
    pub slot_id: u8,
    pub brand: String,
    pub material: String,
    pub color: String,
    pub nozzle_temp: u16,
    pub bed_temp: u16,
}

pub struct BambuMqttClient {
    #[allow(dead_code)]
    client_id: String,
}

impl BambuMqttClient {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            client_id: format!("spoolsync_{}", uuid::Uuid::new_v4()),
        })
    }

    pub fn test_connection(&self, config: BambuPrinterConfig) -> Result<String, String> {
        println!("\n🔌 MQTT CONNECTION TEST");
        println!("═══════════════════════════════════════");
        println!("Printer Name: {}", config.name);
        println!("IP Address: {}", config.ip_address);
        println!("Serial: {}", config.serial_number);
        println!("Client ID: {}", self.client_id);
        println!("═══════════════════════════════════════\n");

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        rt.block_on(async {
            println!("📡 Creating MQTT client...");
            let (client, mut event_loop) = match self.create_mqtt_client(&config).await {
                Ok(result) => {
                    println!("✅ MQTT client created successfully");
                    result
                }
                Err(e) => {
                    eprintln!("❌ Failed to create MQTT client: {}", e);
                    return Err(e);
                }
            };

            println!("⏳ Waiting for connection events (5s timeout)...");
            let result = tokio::time::timeout(Duration::from_secs(5), async {
                let mut event_count = 0;
                while let Ok(notification) = event_loop.poll().await {
                    event_count += 1;
                    println!("📨 Event #{}: {:?}", event_count, notification);
                    
                    match notification {
                        Event::Incoming(Packet::ConnAck(connack)) => {
                            println!("✅ ConnAck received: {:?}", connack);
                            client.disconnect().await.ok();
                            return Ok(format!(
                                "Successfully connected to printer '{}'",
                                config.name
                            ));
                        }
                        Event::Incoming(packet) => {
                            println!("📦 Incoming packet: {:?}", packet);
                        }
                        Event::Outgoing(packet) => {
                            println!("📤 Outgoing packet: {:?}", packet);
                        }
                    }
                }
                Err("Event loop ended without ConnAck".to_string())
            })
            .await;

            match result {
                Ok(Ok(success)) => {
                    println!("\n✅ Connection test successful!");
                    Ok(success)
                }
                Ok(Err(e)) => {
                    eprintln!("\n❌ Connection failed: {}", e);
                    Err(e)
                }
                Err(_) => {
                    eprintln!("\n⏱️ Connection timeout after 5 seconds");
                    eprintln!("Possible issues:");
                    eprintln!("  - Printer IP address incorrect or unreachable");
                    eprintln!("  - Access code is wrong");
                    eprintln!("  - Printer is offline or MQTT is disabled");
                    eprintln!("  - Firewall blocking port 8883");
                    eprintln!("  - TLS certificate issues");
                    Err("Connection timeout".to_string())
                }
            }
        })
    }

    pub fn sync_filament(
        &self,
        config: BambuPrinterConfig,
        command: FilamentSyncCommand,
    ) -> Result<String, String> {
        println!("\n🧵 MQTT FILAMENT SYNC");
        println!("═══════════════════════════════════════");
        println!("Printer: {} ({})", config.name, config.ip_address);
        println!("Slot: {}", command.slot_id);
        println!("Brand: {}", command.brand);
        println!("Material: {}", command.material);
        println!("Color: {}", command.color);
        println!("═══════════════════════════════════════\n");

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        rt.block_on(async {
            let (client, mut event_loop) = self.create_mqtt_client(&config).await?;

            tokio::spawn(async move {
                while let Ok(event) = event_loop.poll().await {
                    println!("📨 MQTT Event: {:?}", event);
                }
            });

            let payload = serde_json::json!({
                "print": {
                    "sequence_id": "0",
                    "command": "ams_filament_setting",
                    "ams_id": 0,
                    "tray_id": command.slot_id,
                    "tray_info_idx": format!("{}", command.slot_id),
                    "tray_color": command.color.replace('#', ""),
                    "nozzle_temp_min": command.nozzle_temp,
                    "nozzle_temp_max": command.nozzle_temp + 10,
                    "tray_type": command.material,
                }
            });

            let topic = format!("device/{}/request", config.serial_number);
            println!("📤 Publishing to topic: {}", topic);
            println!("📦 Payload: {}", payload);

            client
                .publish(
                    topic,
                    QoS::AtLeastOnce,
                    false,
                    payload.to_string().as_bytes(),
                )
                .await
                .map_err(|e| format!("Failed to publish: {}", e))?;

            println!("✅ Message published, waiting 1s...");
            tokio::time::sleep(Duration::from_secs(1)).await;
            client.disconnect().await.ok();

            Ok(format!(
                "Synced {} {} to AMS slot {}",
                command.brand, command.material, command.slot_id
            ))
        })
    }

    async fn create_mqtt_client(
        &self,
        config: &BambuPrinterConfig,
    ) -> Result<(AsyncClient, rumqttc::EventLoop), String> {
        println!("🔧 Configuring MQTT options...");
        println!("   Host: {}:8883", config.ip_address);
        println!("   Username: bblp");
        println!("   Client ID: {}", self.client_id);

        let mut mqtt_options = MqttOptions::new(&self.client_id, &config.ip_address, 8883);
        mqtt_options.set_keep_alive(Duration::from_secs(30));
        mqtt_options.set_credentials("bblp", &config.access_code);

        println!("🔐 Loading TLS certificates...");
        let mut root_store = rumqttc::tokio_rustls::rustls::RootCertStore::empty();
        
        let cert_result = rustls_native_certs::load_native_certs();
        
        let mut loaded_certs = 0;
        for cert in cert_result.certs {
            if root_store.add(cert).is_ok() {
                loaded_certs += 1;
            }
        }
        
        println!("   ✅ Loaded {} system certificates", loaded_certs);
        
        if !cert_result.errors.is_empty() {
            eprintln!("   ⚠️ Certificate load warnings: {} errors", cert_result.errors.len());
            for err in &cert_result.errors {
                eprintln!("      - {:?}", err);
            }
        }

        let client_config = rumqttc::tokio_rustls::rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        mqtt_options.set_transport(Transport::Tls(TlsConfiguration::Rustls(
            Arc::new(client_config),
        )));

        println!("📡 Creating async MQTT client...");
        let (client, event_loop) = AsyncClient::new(mqtt_options, 10);
        println!("✅ Client and event loop created\n");

        Ok((client, event_loop))
    }
}
