use today::events::Category;
use today::events::Event;
use today::providers::{EventProvider, TestEventProvider};

fn main() {
    let mut events: Vec<Event> = Vec::new();

    let provider = TestEventProvider::new();
    provider.get_events(&mut events);

    for event in events {
        println!("{}", event);
    }
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
