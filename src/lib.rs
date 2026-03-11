pub mod events;
pub mod providers;

use serde::Deserialize;
use std::error::Error;
use std::path::Path;
use chrono::Local;
use chrono::NaiveDate;
use chrono::Datelike;

use crate::events::MonthDay;
use crate::events::Event;
use crate::providers::EventProvider;
use crate::providers::TestEventProvider;

#[derive(Deserialize, Debug)]
pub struct ProviderConfig {
    name: String,
    kind: String,
    resource: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    providers: Vec<ProviderConfig>,
}
fn create_providers(config: &Config, config_path: &Path) -> Vec<Box<dyn EventProvider>> {
    let mut providers: Vec<Box<dyn EventProvider>> = Vec::new();
    for cfg in config.providers.iter() {
        let path = config_path.join(&cfg.resource);
        match cfg.kind.as_str() {
            //provider sisällöt
            _ => {
                eprintln!("Unable to make provider: {:?}", cfg);
            }
        }
    }

    let test_provider = TestEventProvider::new();
    providers.push(Box::new(test_provider));

    providers
}

pub fn run(config: &Config, config_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut events: Vec<Event> = Vec::new();
    let providers = create_providers(config, config_path);
    let mut count = 0;
    for provider in providers {
        provider.get_events(&mut events); // polymorphism at work!
        let new_count = events.len();
        println!(
            "Got {} events from provider '{}'",
            new_count - count,
            provider.name()
        );
        count = new_count;
    }
    let today: NaiveDate = Local::now().date_naive();
    let today_month_day = MonthDay::new(today.month(), today.day());
    for event in events {
        if today_month_day == event.month_day() {
            println!("{}", event);
        }
    }
    Ok(())
}
