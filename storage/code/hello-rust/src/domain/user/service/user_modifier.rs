use crate::domain::user::User;

pub trait UserModifier {
    fn modify(&self, user: User);
}