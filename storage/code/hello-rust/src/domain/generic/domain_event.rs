use crate::Id;

pub trait DomainEvent {
    fn get_id(&self) -> &Id;
}