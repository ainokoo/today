use crate::events::Event;
pub mod testprovider;
pub use testprovider::TestEventProvider;

pub trait EventProvider {
    fn name(&self) -> String;
    fn get_events(&self, events: &mut Vec<Event>);
}
