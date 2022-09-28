use crate::domain::user::User;

pub trait SaveUser {
    fn save(&self, user: User);
}