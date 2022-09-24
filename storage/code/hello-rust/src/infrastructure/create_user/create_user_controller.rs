use crate::application::create_user::create_user_command::CreateUserCommand;
use crate::application::create_user::create_user_use_case::CreateUserUseCase;

pub struct CreateUserController<T: CreateUserUseCase> {
    pub create_user_use_case: T,
}

impl<T: CreateUserUseCase> CreateUserController<T> {
    //https://practice.rs/generics-traits/traits.html
    //https://github.com/sunface/rust-by-practice/blob/master/solutions/generics-traits/generics.md
    pub fn create(&self, create_user_command: CreateUserCommand) {
        &self.create_user_use_case.create(create_user_command);
    }
}
