use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use mqtt_sqlite::database::get_connection_pool;
use mqtt_sqlite::database::run_migrations;
use mqtt_sqlite::models::Configuration;
use mqtt_sqlite::models::Payload;
use mqtt_sqlite::schema::*;

use paho_mqtt::{Client, ConnectOptionsBuilder, Message};
use std::env;
use std::fs::read_to_string;
use std::{thread, time::Duration};

fn main() {
    dotenv().ok();
    let config_url = env::var("CONFIG_URL").expect("CONFIG_URL does not exist in .env");

    let pool: Pool<ConnectionManager<SqliteConnection>> = get_connection_pool();
    let connection: &mut SqliteConnection = &mut pool.get().unwrap();

    match run_migrations(connection) {
        Ok(_) => {
            println!("Creating database file and running migrations.");
        }
        Err(e) => {
            panic!("Error while creating database: {}", e);
        }
    }

    let configuration_file = read_to_string(config_url).unwrap_or_else(|error| {
        panic!("Problem opening the file: {:?}", error);
    });

    let configuration: Configuration = serde_json::from_str(&configuration_file)
        .unwrap_or_else(|error| {
            panic!(
                "Unable to read devices from configuration file: {:?}",
                error
            );
        });

    let broker = format!(
        "{}://{}:{}",
        configuration.protocol, configuration.host, configuration.port
    );

    let mqtt_client = Client::new(broker).unwrap_or_else(|error| {
        panic!("Problem creating the client: {:?}", error);
    });

    let conn_opts = ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(30))
        .clean_session(true)
        .finalize();

    match mqtt_client.connect(conn_opts) {
        Ok(_) => {
            println!("Connected to MQTT broker.");
            subscribe(&mqtt_client, &configuration);
        }
        Err(e) => {
            panic!("Unable to connect:\n\t{:?}", e);
        }
    }

    let rx: paho_mqtt::Receiver<Option<paho_mqtt::Message>> =
        mqtt_client.start_consuming();

    for message in rx.iter() {
        match message {
            Some(message) => {
                create_message(connection, message);
            }
            None if !mqtt_client.is_connected() => {
                if reconnect(&mqtt_client) {
                    println!("Resubscribing after reconnection...");
                    subscribe(&mqtt_client, &configuration);
                } else {
                    break;
                }
            }
            _ => (),
        };
    }

    if mqtt_client.is_connected() {
        println!("Disconnecting");
        mqtt_client
            .unsubscribe("zigbee2mqtt")
            .unwrap_or_else(|err| {
                panic!("Error unsubscribing during disconnect attempt {:?}", err);
            });
        mqtt_client.disconnect(None).unwrap_or_else(|err| {
            panic!("Error disconnecting from the client {:?}", err);
        });
    }
    println!("Exiting");
}

fn reconnect(cli: &Client) -> bool {
    println!("Connection lost. Waiting to retry connection...");
    for _ in 0..12 {
        thread::sleep(Duration::from_millis(10000));
        if cli.reconnect().is_ok() {
            println!("Successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect after several attempts.");
    false
}

fn subscribe(cli: &Client, configuration: &Configuration) {
    let mut topics: Vec<String> = vec![];
    let mut topics_qos: Vec<i32> = vec![];

    for device in &configuration.devices {
        topics.push(device.topic.clone());
        topics_qos.push(device.qos);
    }

    match cli.subscribe_many(&topics, &topics_qos) {
        Ok(_) => println!("Successfully subscribed to {:?}", topics),
        Err(e) => {
            panic!("Error subscribing to topic: {:?}", e);
        }
    }
}

fn create_message(connection: &mut SqliteConnection, message: Message) {
    let payload: Payload = serde_json::from_str(&message.payload_str()).unwrap();

    let new_message = Payload {
        friendly_name: Some(message.topic().to_string()),
        ..payload
    };

    diesel::insert_into(logs::table)
        .values(new_message)
        .execute(connection)
        .unwrap_or_else(|error| {
            panic!("Error saving the payload to database.: {:?}", error);
        });
}
