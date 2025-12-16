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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AMSTrayInfo {
    pub tray_id: u8,
    pub tray_type: String,
    pub tray_color: String,
    pub nozzle_temp_min: u16,
    pub nozzle_temp_max: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AMSStatus {
    pub ams_id: u8,
    pub trays: Vec<AMSTrayInfo>,
}

#[derive(Debug)]
struct NoCertVerification;

impl rumqttc::tokio_rustls::rustls::client::danger::ServerCertVerifier for NoCertVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rumqttc::tokio_rustls::rustls::pki_types::CertificateDer,
        _intermediates: &[rumqttc::tokio_rustls::rustls::pki_types::CertificateDer],
        _server_name: &rumqttc::tokio_rustls::rustls::pki_types::ServerName,
        _ocsp_response: &[u8],
        _now: rumqttc::tokio_rustls::rustls::pki_types::UnixTime,
    ) -> Result<rumqttc::tokio_rustls::rustls::client::danger::ServerCertVerified, rumqttc::tokio_rustls::rustls::Error> {
        Ok(rumqttc::tokio_rustls::rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &rumqttc::tokio_rustls::rustls::pki_types::CertificateDer,
        _dss: &rumqttc::tokio_rustls::rustls::DigitallySignedStruct,
    ) -> Result<rumqttc::tokio_rustls::rustls::client::danger::HandshakeSignatureValid, rumqttc::tokio_rustls::rustls::Error> {
        Ok(rumqttc::tokio_rustls::rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &rumqttc::tokio_rustls::rustls::pki_types::CertificateDer,
        _dss: &rumqttc::tokio_rustls::rustls::DigitallySignedStruct,
    ) -> Result<rumqttc::tokio_rustls::rustls::client::danger::HandshakeSignatureValid, rumqttc::tokio_rustls::rustls::Error> {
        Ok(rumqttc::tokio_rustls::rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rumqttc::tokio_rustls::rustls::SignatureScheme> {
        vec![
            rumqttc::tokio_rustls::rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rumqttc::tokio_rustls::rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rumqttc::tokio_rustls::rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rumqttc::tokio_rustls::rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rumqttc::tokio_rustls::rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rumqttc::tokio_rustls::rustls::SignatureScheme::ECDSA_NISTP521_SHA512,
            rumqttc::tokio_rustls::rustls::SignatureScheme::RSA_PSS_SHA256,
            rumqttc::tokio_rustls::rustls::SignatureScheme::RSA_PSS_SHA384,
            rumqttc::tokio_rustls::rustls::SignatureScheme::RSA_PSS_SHA512,
            rumqttc::tokio_rustls::rustls::SignatureScheme::ED25519,
        ]
    }
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

            println!("📤 Subscribing to report topic to trigger connection...");
            let report_topic = format!("device/{}/report", config.serial_number);
            if let Err(e) = client.subscribe(&report_topic, QoS::AtMostOnce).await {
                eprintln!("❌ Failed to subscribe: {}", e);
                return Err(format!("Subscribe failed: {}", e));
            }
            println!("✅ Subscribe request sent\n");

            println!("⏳ Waiting for connection events (10s timeout)...");
            let result = tokio::time::timeout(Duration::from_secs(10), async {
                let mut event_count = 0;
                
                loop {
                    match event_loop.poll().await {
                        Ok(notification) => {
                            event_count += 1;
                            
                            match &notification {
                                Event::Incoming(Packet::ConnAck(connack)) => {
                                    println!("📨 Event #{}: ConnAck received!", event_count);
                                    println!("   Session present: {}", connack.session_present);
                                    println!("   Code: {:?}", connack.code);
                                    client.disconnect().await.ok();
                                    return Ok(format!("Successfully connected to printer '{}'", config.name));
                                }
                                Event::Incoming(packet) => {
                                    println!("📨 Event #{}: Incoming {:?}", event_count, packet);
                                }
                                Event::Outgoing(outgoing) => {
                                    println!("📤 Event #{}: Outgoing {:?}", event_count, outgoing);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Event loop error: {:?}", e);
                            return Err(format!("Connection error: {:?}", e));
                        }
                    }
                }
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
                    eprintln!("\n⏱️ Connection timeout after 10 seconds");
                    eprintln!("\nPossible issues:");
                    eprintln!("  1. Printer IP address incorrect or unreachable");
                    eprintln!("     → Ping test: ping {}", config.ip_address);
                    eprintln!("  2. Access code is wrong");
                    eprintln!("     → Check in printer: Settings → Network → Access Code");
                    eprintln!("  3. Printer MQTT is disabled or printer is offline");
                    eprintln!("  4. Firewall blocking port 8883");
                    Err("Connection timeout - no response from printer".to_string())
                }
            }
        })
    }

    pub fn get_ams_status(&self, config: BambuPrinterConfig) -> Result<Vec<AMSStatus>, String> {
        println!("\n📊 GET AMS STATUS");
        println!("═══════════════════════════════════════");
        println!("Printer: {} ({})", config.name, config.ip_address);
        println!("═══════════════════════════════════════\n");

        let rt = tokio::runtime::Runtime::new()
            .map_err(|e| format!("Failed to create runtime: {}", e))?;

        rt.block_on(async {
            let (client, mut event_loop) = self.create_mqtt_client(&config).await?;
            
            let report_topic = format!("device/{}/report", config.serial_number);
            client.subscribe(&report_topic, QoS::AtMostOnce).await
                .map_err(|e| format!("Subscribe failed: {}", e))?;

            println!("⏳ Waiting for AMS status message (30s timeout)...");
            println!("💡 Make sure printer is powered on and connected\n");
            
            let result = tokio::time::timeout(Duration::from_secs(30), async {
                let mut message_count = 0;
                loop {
                    match event_loop.poll().await {
                        Ok(Event::Incoming(Packet::Publish(publish))) => {
                            message_count += 1;
                            if let Ok(payload_str) = String::from_utf8(publish.payload.to_vec()) {
                                println!("📨 Message #{}: Received {} bytes", message_count, payload_str.len());
                                
                                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&payload_str) {
                                    if let Some(print_obj) = json.get("print") {
                                        println!("   ✅ Found 'print' object");
                                        
                                        if let Some(ams_obj) = print_obj.get("ams") {
                                            println!("   ✅ Found 'ams' object");
                                            
                                            if let Some(ams_array) = ams_obj.get("ams").and_then(|a| a.as_array()) {
                                                println!("   ✅ Found 'ams.ams' array with {} units", ams_array.len());
                                                let mut statuses = Vec::new();
                                                
                                                for (ams_idx, ams_unit) in ams_array.iter().enumerate() {
                                                    if let Some(tray_array) = ams_unit.get("tray").and_then(|t| t.as_array()) {
                                                        println!("   📦 AMS {}: {} trays", ams_idx, tray_array.len());
                                                        let mut trays = Vec::new();
                                                        
                                                        for (tray_idx, tray_obj) in tray_array.iter().enumerate() {
                                                            let tray_type = tray_obj.get("tray_type").and_then(|t| t.as_str()).unwrap_or("").to_string();
                                                            let tray_color = tray_obj.get("tray_color").and_then(|c| c.as_str()).unwrap_or("000000").to_string();
                                                            let nozzle_temp_min = tray_obj.get("nozzle_temp_min").and_then(|t| t.as_u64()).unwrap_or(0) as u16;
                                                            let nozzle_temp_max = tray_obj.get("nozzle_temp_max").and_then(|t| t.as_u64()).unwrap_or(0) as u16;
                                                            
                                                            if !tray_type.is_empty() {
                                                                println!("      Tray {}: {} (#{}) {}°C-{}°C", 
                                                                    tray_idx, tray_type, tray_color, nozzle_temp_min, nozzle_temp_max);
                                                                trays.push(AMSTrayInfo {
                                                                    tray_id: tray_idx as u8,
                                                                    tray_type,
                                                                    tray_color,
                                                                    nozzle_temp_min,
                                                                    nozzle_temp_max,
                                                                });
                                                            }
                                                        }
                                                        
                                                        if !trays.is_empty() {
                                                            statuses.push(AMSStatus {
                                                                ams_id: ams_idx as u8,
                                                                trays,
                                                            });
                                                        }
                                                    }
                                                }
                                                
                                                if !statuses.is_empty() {
                                                    println!("\n✅ Retrieved status for {} AMS unit(s)", statuses.len());
                                                    client.disconnect().await.ok();
                                                    return Ok(statuses);
                                                }
                                            } else {
                                                println!("   ⚠️ 'ams.ams' not found or not an array");
                                            }
                                        } else {
                                            println!("   ⚠️ 'ams' object not found in 'print'");
                                        }
                                    }
                                }
                            }
                        }
                        Ok(_) => {},
                        Err(e) => {
                            eprintln!("❌ Event loop error: {:?}", e);
                            return Err(format!("Connection error: {:?}", e));
                        }
                    }
                }
            }).await;

            client.disconnect().await.ok();

            match result {
                Ok(Ok(statuses)) => Ok(statuses),
                Ok(Err(e)) => Err(e),
                Err(_) => {
                    println!("\n⚠️ No AMS status received within 30s");
                    println!("   Make sure:");
                    println!("   1. Printer is powered ON");
                    println!("   2. Printer is connected to network");
                    println!("   3. AMS is connected to printer");
                    Ok(vec![])
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

            let event_task = tokio::spawn(async move {
                while let Ok(event) = event_loop.poll().await {
                    println!("📨 MQTT Event: {:?}", event);
                }
            });

            tokio::time::sleep(Duration::from_millis(500)).await;

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
            event_task.abort();

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

        println!("🔐 Configuring TLS (accepting self-signed Bambu Lab certs)...");
        
        let client_config = rumqttc::tokio_rustls::rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(NoCertVerification))
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
