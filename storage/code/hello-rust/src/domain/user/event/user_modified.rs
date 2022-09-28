use crate::domain::generic::domain_event::DomainEvent;
use crate::Id;

struct UserModified {
    pub id: Id
}

impl DomainEvent for UserModified {
    fn get_id(&self)->&Id {
        return &self.id;
    }
}