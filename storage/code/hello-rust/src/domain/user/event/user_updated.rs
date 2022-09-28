use crate::domain::generic::domain_event::DomainEvent;
use crate::Id;

struct UserUpdated {
    pub id: Id
}

impl DomainEvent for UserUpdated {
    fn get_id(&self)->&Id {
        return &self.id;
    }
}