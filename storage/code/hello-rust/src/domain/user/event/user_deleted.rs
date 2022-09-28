use crate::domain::generic::domain_event::DomainEvent;
use crate::Id;

struct UserDeleted {
    pub id: Id
}

impl DomainEvent for UserDeleted {
    fn get_id(&self)->&Id {
        return &self.id;
    }
}