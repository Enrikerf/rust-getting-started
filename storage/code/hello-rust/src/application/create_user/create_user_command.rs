use crate::{Id, Name};

pub struct CreateUserCommand{
    pub(crate) id: Id,
    pub(crate) name: Name,
}
