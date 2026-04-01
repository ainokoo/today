use crate::events::{Category, Event};
use crate::filters::EventFilter;
use crate::providers::EventProvider;

use chrono::NaiveDate;

pub struct TestEventProvider;

impl TestEventProvider {
    pub fn new() -> Self {
        Self
    }
}

impl EventProvider for TestEventProvider {
    fn name(&self) -> String {
        String::from("Test Event Provider")
    }

    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>) {
        let all_events = vec![
            Event::new_singular(
                NaiveDate::from_ymd_opt(2009, 1, 15).unwrap(),
                String::from("Sully lands on the Hudson River"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1913, 1, 16).unwrap(),
                String::from("Prohibition of alcohol is ratified in US"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1905, 1, 22).unwrap(),
                String::from("Russian revolution: Bloody Sunday"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1968, 1, 22).unwrap(),
                String::from("NASA's first unmanned test flight"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(2018, 1, 22).unwrap(),
                String::from("Netflix becomes the largest digital media"),
                Category::from_primary("FunFact"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1978, 1, 19).unwrap(),
                String::from("Last VW Beetle made in Germany"),
                Category::from_primary("FunFact"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1997, 1, 20).unwrap(),
                String::from("Daft Punk released their album 'Homework'"),
                Category::from_primary("FunFact"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1944, 1, 17).unwrap(),
                String::from("Nazis began evacuation of Auschwitz"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1951, 1, 18).unwrap(),
                String::from("First use of lie detector in the Netherlands"),
                Category::from_primary("FunFact"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1932, 1, 21).unwrap(),
                String::from("USSR and Finland stop non-attack treaty"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1809, 1, 19).unwrap(),
                String::from("Edgar Allan Poe was born"),
                Category::from_primary("Birthday"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(2020, 1, 23).unwrap(),
                String::from("China locks down the city of Wuhan"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1987, 1, 24).unwrap(),
                String::from("Luis Suárez was born"),
                Category::from_primary("Birthday"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(2011, 1, 25).unwrap(),
                String::from("Egyptian Revolution of 2011 begins"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1925, 1, 26).unwrap(),
                String::from("Paul Newman was born"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1892, 1, 29).unwrap(),
                String::from("The Coca Cola Company is incorporated in Atlanta"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1945, 1, 27).unwrap(),
                String::from("Soviet troops liberate Auschwitz and Birkenau Concentration camps"),
                Category::from_primary("Historical"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1981, 1, 28).unwrap(),
                String::from("Elijah Wood was born"),
                Category::from_primary("Birthday"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1302, 3, 11).unwrap(),
                String::from("Romeo and Juliet's wedding day, according to Shakespeare"),
                Category::from_primary("FunFact"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1922, 3, 25).unwrap(),
                String::from("Birthday of Eileen Ford"),
                Category::from_primary("Birthday"),
            ),
            Event::new_singular(
                NaiveDate::from_ymd_opt(1976, 4, 1).unwrap(),
                String::from("Steve Wozniak and Steve Jobs found Apple Computer in the garage of Jobs' parents' house in Cupertino, California"),
                Category::from_primary("Historical"),
            ),            
        ];

        for event in all_events {
            if filter.accepts(&event) {
                events.push(event);
            }
        }
    }
}
/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provider_sets_18_events() {
        let provider = TestEventProvider::new();
        let mut events = Vec::new();

        provider.get_events(&mut events);
        assert_eq!(events.len(), 18, "Provider should add 18 events");
    }

    #[test]
    fn contains_sully_event() {
        let provider = TestEventProvider::new();
        let mut events = Vec::new();
        provider.get_events(&mut events);

        let found = events
            .iter()
            .any(|e| e.description == "Sully lands on the Hudson River");
        assert!(found, "Should contain the Sully event");
    }
}
    */
