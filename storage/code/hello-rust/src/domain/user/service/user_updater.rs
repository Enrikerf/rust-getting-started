use crate::domain::user::User;

pub trait UserUpdater {
    fn update(&self, user: User);
}