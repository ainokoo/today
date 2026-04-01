use crate::events::Event;
use crate::filters::EventFilter;

pub mod sqlite;
pub mod testprovider;

pub use testprovider::TestEventProvider;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, filter: &EventFilter, events: &mut Vec<Event>);
}
