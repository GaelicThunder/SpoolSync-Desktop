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
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        rt.block_on(async {
            let (client, mut event_loop) = self.create_mqtt_client(&config).await?;

            tokio::time::timeout(Duration::from_secs(5), async {
                while let Ok(notification) = event_loop.poll().await {
                    if let Event::Incoming(Packet::ConnAck(_)) = notification {
                        client.disconnect().await.ok();
                        return Ok(format!(
                            "Successfully connected to printer '{}'",
                            config.name
                        ));
                    }
                }
                Err("Connection timeout".to_string())
            })
            .await
            .map_err(|_| "Connection timeout".to_string())?
        })
    }

    pub fn sync_filament(
        &self,
        config: BambuPrinterConfig,
        command: FilamentSyncCommand,
    ) -> Result<String, String> {
        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        rt.block_on(async {
            let (client, mut event_loop) = self.create_mqtt_client(&config).await?;

            tokio::spawn(async move {
                while event_loop.poll().await.is_ok() {}
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
            client
                .publish(
                    topic,
                    QoS::AtLeastOnce,
                    false,
                    payload.to_string().as_bytes(),
                )
                .await
                .map_err(|e| format!("Failed to publish: {}", e))?;

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
        let mut mqtt_options = MqttOptions::new(&self.client_id, &config.ip_address, 8883);
        mqtt_options.set_keep_alive(Duration::from_secs(30));
        mqtt_options.set_credentials("bblp", &config.access_code);

        let mut root_store = rumqttc::tokio_rustls::rustls::RootCertStore::empty();
        
        let cert_result = rustls_native_certs::load_native_certs();
        
        for cert in cert_result.certs {
            root_store.add(cert).ok();
        }
        
        if !cert_result.errors.is_empty() {
            eprintln!("Warning: Some certificates failed to load: {:?}", cert_result.errors);
        }

        let client_config = rumqttc::tokio_rustls::rustls::ClientConfig::builder()
            .with_root_certificates(root_store)
            .with_no_client_auth();

        mqtt_options.set_transport(Transport::Tls(TlsConfiguration::Rustls(
            Arc::new(client_config),
        )));

        let (client, event_loop) = AsyncClient::new(mqtt_options, 10);

        Ok((client, event_loop))
    }
}
