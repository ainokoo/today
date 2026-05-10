pub mod events;
pub mod filters;
pub mod providers;
pub mod birthday;

use chrono::Datelike;
use chrono::Local;
use chrono::NaiveDate;
use serde::Deserialize;
use std::error::Error;
use std::path::Path;
use crate::birthday::handle_birthday;
use crate::events::Event;
use crate::events::MonthDay;
use crate::providers::EventProvider;
use crate::providers::{
    textfile::TextFileProvider,
    sqlite::SQLiteProvider,
    csv::CSVFileProvider,
    web::WebProvider
};
use crate::filters::EventFilter;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    pub name: String,
    pub kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub providers: Vec<ProviderConfig>,
}
fn create_providers(config: &Config, config_path: &Path) -> Vec<Box<dyn EventProvider>> {
    let mut providers: Vec<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            //provider sisällöt
            "sqlite" => {
                let provider = SQLiteProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "text" => {
                let provider = TextFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "csv" => {
                let provider = CSVFileProvider::new(&cfg.name, &path);
                providers.push(Box::new(provider));
            },
            "web" => {
                let provider = WebProvider::new(&cfg.name, &cfg.resource);
                providers.push(Box::new(provider));
            },
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }
    providers
}

pub fn run(config: &Config, config_path: &Path, filter: &EventFilter) -> Result<(), Box<dyn Error>> {
    handle_birthday();
    let mut events: Vec<Event> = Vec::new();

    let today: NaiveDate = Local::now().date_naive();
    let _today_month_day = MonthDay::new(today.month(), today.day());

    let providers = create_providers(config, config_path);

    let mut count = 0;
    for provider in providers {
        provider.get_events(&filter, &mut events);
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'",
            new_count - count,
            provider.name()
        );
        count = new_count;
    }

    for event in &events {
        println!("{:?} - {}, category: {}", event.month_day(), event.description(), event.category());
    }

    Ok(())
}

pub fn add_event(config: &Config, config_path: &Path, provider_name: &str, event: &Event) {
    let providers = create_providers(config, config_path);

    let mut provider: Option<&dyn EventProvider> = None;
    for p in &providers {
        if p.name() == provider_name {
            provider = Some(p.as_ref());
            break;
        }
    }

    match provider {
        Some(p) => {
            if p.is_add_supported() {
                let _ = p.add_event(event);
            } else {
                eprintln!("Adding events is not supported for provider '{}'", p.name());
            }
        },
        None => {
            eprintln!("Unknown event provider '{}'", provider_name);
        }
    }
}