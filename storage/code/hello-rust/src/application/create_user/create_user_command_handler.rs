use uuid::Uuid;

use crate::{Id, Name};
use crate::application::create_user::create_user_command::CreateUserCommand;
use crate::application::create_user::create_user_use_case::CreateUserUseCase;
use crate::domain::user::User;

pub struct CreateUserCommandHandler;

impl CreateUserCommandHandler {
    pub fn new() -> Self {
        return Self {};
    }
}

impl CreateUserUseCase for CreateUserCommandHandler {
    fn create(&self, create_user_command: CreateUserCommand) -> Id {
        let new_user = User::new(create_user_command.id, create_user_command.name);
        return new_user.id;
    }
}
