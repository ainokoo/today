use crate::events::{Category, Event, MonthDay};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterOption {
    MonthDay(MonthDay),
    Category(Category),
    Text(String),
}

pub struct EventFilter {
    month_day: Option<MonthDay>,
    description_contains: Option<String>,
    category_matches: Option<Category>,
}

impl EventFilter {
    pub fn month_day(&self) -> Option <MonthDay> {
        self.month_day.clone()
    }

    pub fn description_contains(&self) -> Option<String> {
        self.description_contains.clone()
    }

    pub fn category_matches(&self) -> Option<Category> {
        self.category_matches.clone()
    }    

    pub fn accepts(&self, event: &Event) -> bool {
        if let Some(month_day) = &self.month_day {
            if event.month_day() != *month_day {
                return false;
            }
        }

        if let Some(ref text) = self.description_contains {
            if !event.description().to_lowercase().contains(text) {
                return false;
            }
        }

        if let Some(ref filter_cat) = self.category_matches {
            if event.category() != *filter_cat {
                return false;
            }
        }

        true
    }
}

pub struct FilterBuilder {
    month_day: Option<MonthDay>,
    description_contains: Option<String>,
    category_matches: Option<Category>,
}

impl FilterBuilder {
    pub fn new() -> Self {
        Self {
            month_day: None,
            description_contains: None,
            category_matches: None,
        }
    }

    pub fn month_day(mut self, month_day: MonthDay) -> Self {
        self.month_day = Some(month_day);
        self
    }

    pub fn description_contains(mut self, text: impl Into<String>) -> Self {
        self.description_contains = Some(text.into().to_lowercase());
        self
    }

    pub fn category_matches(mut self, category: &Category) -> Self {
        self.category_matches = Some(category.clone());
        self
    }

    pub fn build(self) -> EventFilter {
        EventFilter {
            month_day: self.month_day,
            description_contains: self.description_contains,
            category_matches: self.category_matches,
        }
    }    
}
