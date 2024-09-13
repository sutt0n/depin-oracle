mod config;
pub mod error;

pub use config::*;

use rumqttc::{AsyncClient, MqttOptions, TlsConfiguration, Transport};
use std::time::Duration;

use crate::app::OracleApp;

use self::error::BrokerError;

#[derive(Clone)]
pub struct Broker {
    _config: BrokerConfig,
    options: MqttOptions,
}

impl Broker {
    pub async fn init(config: BrokerConfig) -> anyhow::Result<Self, BrokerError> {
        let mut mqtt_options = MqttOptions::new("Test1", config.uri.clone(), config.port);
        mqtt_options.set_keep_alive(Duration::from_secs(config.keep_alive));

        if !config.ca_cert.is_empty() {
            let ca = std::fs::read(config.ca_cert.clone()).unwrap();
            let client_key = std::fs::read(config.client_key.clone()).unwrap();
            let client_cert = std::fs::read(config.client_cert.clone()).unwrap();

            let transport = Transport::Tls(TlsConfiguration::Simple {
                ca,
                alpn: None,
                client_auth: Some((client_cert, client_key)),
            });

            mqtt_options.set_transport(transport);
        }

        Ok(Self {
            _config: config,
            options: mqtt_options.clone(),
        })
    }

    pub async fn run(&mut self, app: OracleApp) -> anyhow::Result<(), BrokerError> {
        println!(
            "Connecting to broker: {}:{}",
            self._config.uri.clone(),
            self._config.port.clone()
        );

        let (client, mut eventloop) = AsyncClient::new(self.options.clone(), 10);

        client
            .subscribe(self._config.topic.clone(), rumqttc::QoS::AtMostOnce)
            .await?;

        let _ = tokio::spawn(async move {
            loop {
                let event = eventloop.poll().await;

                if let rumqttc::Event::Incoming(incoming) = event.unwrap() {
                    println!("Incoming Event: {:?}", incoming);

                    if let rumqttc::Packet::Publish(publish) = incoming {
                        let payload = publish.payload.clone();
                        // bytes to Vec<u8>
                        let payload = payload.to_vec();

                        app.handle_drone_mqtt(payload).await;
                    }
                }
            }
        })
        .await;

        Ok(())
    }
}
