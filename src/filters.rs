use crate::events::{Category, Event, MonthDay};

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOption {
    MonthDay(MonthDay),
    Category(Category),
    Text(String),
}

pub struct EventFilter {
    options: HashSet<FilterOption>,
}

impl EventFilter {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }

    pub fn accepts(&self, event: &Event) -> bool {
        if self.options.is_empty() {
            return true;
        }
        let mut results: Vec<bool> = Vec::new();

        for option in self.options.iter() {
            let result = match option {
                FilterOption::MonthDay(month_day) => *month_day == event.month_day(),
                FilterOption::Category(category) => *category == event.category(),
                FilterOption::Text(text) => event.description().contains(text),
            };
            results.push(result);
        }

        results.iter().all(|&option| option)
    }
}

pub struct FilterBuilder {
    options: HashSet<FilterOption>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            options: HashSet::new(),
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> FilterBuilder {
        self.options.insert(FilterOption::MonthDay(month_day));
        self
    }

    pub fn category(mut self, category: Category) -> FilterBuilder {
        self.options.insert(FilterOption::Category(category));
        self
    }

    pub fn text(mut self, text: String) -> FilterBuilder {
        self.options.insert(FilterOption::Text(text));
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            options: self.options,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Datelike, Local, NaiveDate};

    #[test]
    fn filter_accepts_date() {
        let bday_category = Category::new("Birthday", "");
        let today: NaiveDate = Local::now().date_naive();
        let month_day = MonthDay::new(today.month(), today.day());
        let event = Event::new_singular(today, "Test event".to_string(), bday_category.clone());
        let filter = FilterBuilder::new().month_day(month_day).build();

        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_category() {
        let bday_category = Category::new("Birthday", "");
        let today: NaiveDate = Local::now().date_naive();
        let event = Event::new_singular(today, "Test event".to_string(), bday_category.clone());
        let filter = FilterBuilder::new().category(bday_category).build();

        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_desc() {
        let bday_category = Category::new("Birthday", "");
        let today: NaiveDate = Local::now().date_naive();
        let event_desc = "Test event".to_string();
        let event = Event::new_singular(today, event_desc.clone(), bday_category.clone());
        let filter = FilterBuilder::new().text(event_desc).build();

        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_accepts_all() {
        let bday_category = Category::new("Birthday", "");
        let today: NaiveDate = Local::now().date_naive();
        let month_day = MonthDay::new(today.month(), today.day());
        let event_desc = "Test event".to_string();
        let event = Event::new_singular(today, event_desc.clone(), bday_category.clone());
        let filter = FilterBuilder::new()
            .month_day(month_day)
            .category(bday_category)
            .text(event_desc)
            .build();

        assert!(filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_all() {
        let bday_category = Category::new("Birthday", "");
        let filter_category = Category::new("Funfact", "");

        let filter_date = NaiveDate::from_ymd_opt(2025, 3, 29).unwrap();
        let event_date = NaiveDate::from_ymd_opt(2020, 3, 24).unwrap();
        let month_day = MonthDay::new(filter_date.month(), filter_date.day());

        let event_desc = "Test event".to_string();
        let filter_desc = "Not test event".to_string();

        let event = Event::new_singular(event_date, event_desc.clone(), bday_category.clone());
        let filter = FilterBuilder::new()
            .month_day(month_day)
            .category(filter_category)
            .text(filter_desc)
            .build();

        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_date() {
        let bday_category = Category::new("Birthday", "");

        let filter_date = NaiveDate::from_ymd_opt(2025, 3, 29).unwrap();
        let event_date = NaiveDate::from_ymd_opt(2020, 3, 24).unwrap();
        let month_day = MonthDay::new(filter_date.month(), filter_date.day());

        let event_desc = "Test event".to_string();

        let event = Event::new_singular(event_date, event_desc.clone(), bday_category.clone());
        let filter = FilterBuilder::new().month_day(month_day).build();

        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_category() {
        let bday_category = Category::new("Birthday", "");
        let filter_category = Category::new("Funfact", "");

        let event_date = NaiveDate::from_ymd_opt(2020, 3, 24).unwrap();

        let event_desc = "Test event".to_string();

        let event = Event::new_singular(event_date, event_desc.clone(), bday_category.clone());
        let filter = FilterBuilder::new().category(filter_category).build();

        assert!(!filter.accepts(&event));
    }

    #[test]
    fn filter_rejects_desc() {
        let bday_category = Category::new("Birthday", "");

        let event_date = NaiveDate::from_ymd_opt(2020, 3, 24).unwrap();

        let event_desc = "Test event".to_string();
        let filter_desc = "Not test event".to_string();

        let event = Event::new_singular(event_date, event_desc.clone(), bday_category.clone());
        let filter = FilterBuilder::new().text(filter_desc).build();

        assert!(!filter.accepts(&event));
    }
}
