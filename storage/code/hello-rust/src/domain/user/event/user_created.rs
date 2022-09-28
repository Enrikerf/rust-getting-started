use crate::domain::generic::domain_event::DomainEvent;
use crate::Id;

pub struct UserCreated {
    pub id: Id,
}

impl DomainEvent for UserCreated {
    fn get_id(&self) -> &Id {
        return &self.id;
    }
}