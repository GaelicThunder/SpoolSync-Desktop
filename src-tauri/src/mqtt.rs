use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS, TlsConfiguration, Transport};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BambuPrinterConfig {
    pub ip: String,
    pub serial: String,
    pub access_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilamentSyncCommand {
    pub ams_id: u8,
    pub tray_id: u8,
    pub tray_color: String,
    pub nozzle_temp_min: u16,
    pub nozzle_temp_max: u16,
    pub tray_type: String,
}

pub struct BambuMqttClient {
    runtime: Runtime,
}

impl BambuMqttClient {
    pub fn new() -> Result<Self, String> {
        let runtime = Runtime::new().map_err(|e| format!("Failed to create runtime: {}", e))?;
        Ok(Self { runtime })
    }

    pub fn test_connection(&self, config: BambuPrinterConfig) -> Result<String, String> {
        self.runtime.block_on(async {
            let (_client, mut eventloop) = Self::create_client(&config).await?;
            
            tokio::time::timeout(Duration::from_secs(3), async {
                eventloop.poll().await
            })
            .await
            .map_err(|_| "Connection timeout")??
            .map_err(|e| format!("Connection failed: {}", e))?;
            
            Ok(format!("Successfully connected to printer at {}", config.ip))
        })
    }

    pub fn sync_filament(
        &self,
        config: BambuPrinterConfig,
        command: FilamentSyncCommand,
    ) -> Result<String, String> {
        self.runtime.block_on(async {
            let (mut client, mut eventloop) = Self::create_client(&config).await?;

            let topic = format!("device/{}/request", config.serial);

            let payload = serde_json::json!({
                "print": {
                    "command": "ams_filament_setting",
                    "ams_id": command.ams_id,
                    "tray_id": command.tray_id,
                    "tray_color": command.tray_color,
                    "nozzle_temp_min": command.nozzle_temp_min,
                    "nozzle_temp_max": command.nozzle_temp_max,
                    "tray_type": command.tray_type
                }
            });

            let payload_str = serde_json::to_string(&payload)
                .map_err(|e| format!("Failed to serialize payload: {}", e))?;

            client
                .publish(topic.clone(), QoS::AtLeastOnce, false, payload_str.as_bytes())
                .await
                .map_err(|e| format!("Failed to publish: {}", e))?;

            let result = tokio::time::timeout(Duration::from_secs(5), async {
                while let Ok(event) = eventloop.poll().await {
                    if let Event::Incoming(Packet::PubAck(_)) = event {
                        return Ok::<(), String>(());
                    }
                }
                Err("No acknowledgment received".to_string())
            })
            .await
            .map_err(|_| "Connection timeout".to_string())?
            .map_err(|e| format!("MQTT error: {}", e))?;

            result.map_err(|e| e)?;

            Ok(format!(
                "Successfully synced to AMS {} Tray {}",
                command.ams_id, command.tray_id
            ))
        })
    }

    async fn create_client(
        config: &BambuPrinterConfig,
    ) -> Result<(AsyncClient, rumqttc::EventLoop), String> {
        let mut mqttoptions = MqttOptions::new("spoolsync-desktop", &config.ip, 8883);
        mqttoptions.set_credentials("bblp", &config.access_code);
        mqttoptions.set_keep_alive(Duration::from_secs(30));

        let mut root_store = rumqttc::tokio_rustls::rustls::RootCertStore::empty();
        for cert in rustls_native_certs::load_native_certs()
            .map_err(|e| format!("Failed to load native certs: {}", e))?
        {
            root_store.add(cert).ok();
        }

        let client_config = rumqttc::tokio_rustls::rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        let tls_config = TlsConfiguration::Rustls(Arc::new(client_config));
        mqttoptions.set_transport(Transport::tls_with_config(tls_config));

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

        Ok((client, eventloop))
    }
}
