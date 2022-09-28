use crate::domain::user::User;

pub trait UserCreator {
    fn create(&self, user: User);
}