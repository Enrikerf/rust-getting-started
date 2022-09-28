use crate::domain::generic::filter::Filter;
use crate::domain::user::User;

pub trait FindUsers {
    fn find(&self, filters: Vec<Filter>) -> Vec<User>;
}