
use today::events::Category;
use today::{Config, run};

use dirs;
use std::fs;
use std::path::PathBuf;

use clap::Parser;

use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;

use today::filters::FilterBuilder;
use today::filters::EventFilter;

use today::events::MonthDay;

#[derive(Parser)]
#[command(name = "today")]
struct Args {
    #[arg(short, long, help = "Event date in MMDD format")]
    date: Option<String>,
}

fn main() {

    let args = Args::parse();

    let month_day = if let Some(md) = args.date {
        MonthDay::from_str(&md)
    } else {
        let today: NaiveDate = Local::now().date_naive();
        MonthDay::new(today.month(), today.day())
    };

    let filter: EventFilter = FilterBuilder::new()
        .month_day(month_day)
        .build();

    const APP_NAME: &str = "today";
    let config_path = get_config_path(APP_NAME);
    match config_path {
        Some(path) => {
            let toml_path = path.join(format!("{}.toml", APP_NAME));
            println!("Looking for configuration file '{}'", &toml_path.display());
            let config_str = fs::read_to_string(toml_path).expect("existing configuration file");
            let config: Config = toml::from_str(&config_str).expect("valid configuration file");
            println!("config: {:#?}", config);
            if let Err(e) = run(&config, &path, &filter) {
                eprintln!("Error running problem");
                return;
            }
        }
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
