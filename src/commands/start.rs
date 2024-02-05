use super::init::Database;
use std::io;

pub async fn start() -> io::Result<()> {
    let current_working_directory: std::path::PathBuf =
        std::env::current_dir().expect("Unable to get current directory.");
    let config_file_path: std::path::PathBuf = current_working_directory.join("config.yaml");
    let file_contents: String = std::fs::read_to_string(config_file_path)?;
    let config_contents: serde_yaml::Value =
        serde_yaml::from_str(&file_contents).expect("Unable to parse config file.");
    match config_contents["database"].as_str() {
        Some("Mongodb") => {
            let database: Database = Database::Mongodb {
                uri: config_contents["uri"].as_str().unwrap().to_string(),
            };
            let _a: Result<(), io::Error> = database.run().await;
        }
        Some("Postgres") => {
            let database: Database = Database::Postgres {
                uri: config_contents["uri"].as_str().unwrap().to_string(),
            };
            let _a: Result<(), io::Error> = database.run().await;
        }
        Some("MySQL") => {
            let database: Database = Database::MySQL {
                uri: config_contents["uri"].as_str().unwrap().to_string(),
            };
            let _a: Result<(), io::Error> = database.run().await;
        }
        _ => {
            println!("Database not supported.");
        }
    }
    Ok(())
}
