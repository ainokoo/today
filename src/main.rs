use today::events::Category;
use today::events::Event;
use today::providers::{EventProvider, TestEventProvider};
use today::{run, Config};

use std::path::PathBuf;
use std::fs;
use dirs;

fn main() {

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            println!("Looking for configuration file '{}'", &toml_path.display());
            let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
            let config: Config = toml::from_str(&config_str).expect("valid configuration file");
            println!("config: {:#?}", config);           
            if let Err(e) = today::run(&config, &path) {
                eprintln!("Error: {}", e);
                return;
            }
        },
        None => {
            eprintln!("Unable to configure the application");
            return;
        }
    }
}

fn get_config_path(app_name: &str) -> Option<PathBuf> {
    if let Some(config_dir) = dirs::config_dir() {
        println!("Config directory: '{}'", config_dir.display());

        // Check if our config directory exists
        let config_path = config_dir.join(app_name);
        print!("App config directory: '{}'", config_path.display());

        if !config_path.exists() {
            if let Err(_) = fs::create_dir(&config_path) {
                eprintln!("Unable to create config directory for {}", app_name);
                return None;
            } else {
                print!(" - created");
            }
        } else {
            print!(" - exists");
        }
        println!();

        return Some(config_path);
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::Category;

    #[test]
    fn sets_primary_and_secondary_category() {
        assert_eq!(
            Category::new("programming", "rust"),
            Category {
                primary: "programming".to_string(),
                secondary: Some("rust".to_string()),
            }
        );
    }

    #[test]
    fn just_primary_category() {
        assert_eq!(
            Category::from_primary("CategoryName"),
            Category {
                primary: "CategoryName".to_string(),
                secondary: None,
            }
        );
    }

    #[test]
    fn sets_primary_and_secondary_category_with_slash() {
        assert_eq!(
            Category::from_str("programming/rust"),
            Category {
                primary: "programming".to_string(),
                secondary: Some("rust".to_string()),
            }
        );
    }
}
