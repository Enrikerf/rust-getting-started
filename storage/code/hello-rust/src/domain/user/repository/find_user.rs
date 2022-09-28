use crate::domain::user::User;
use crate::domain::user::vo::id::Id;

pub trait FindUser {
    fn find_by_id(&self, id: Id) -> User;
}