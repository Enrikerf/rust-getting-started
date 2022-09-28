use uuid::Uuid;
use crate::domain::generic::domain_event::DomainEvent;
use crate::domain::user::event::user_created::UserCreated;
use crate::domain::user::vo::id::Id;
use crate::domain::user::vo::name::Name;

pub mod repository;
pub mod service;
pub mod vo;
pub mod event;

pub struct User {
    pub id: Id,
    pub name: Name,
    events: Vec::<Box<dyn DomainEvent>>,
    _secret: (),
}


impl User {
    pub fn new(id: Id, name: Name) -> Self {
        let mut events = Vec::<Box<dyn DomainEvent>>::new();
        events.push(Box::new(UserCreated { id: Id { value: Uuid::new_v4() } }));
        return User {
            id,
            name,
            events,
            _secret: (),
        };
    }
    pub fn add_event(&self) {}
    pub fn get_events(&self) -> &Vec::<Box<dyn DomainEvent>> { return &self.events; }
}