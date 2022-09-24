use crate::application::create_user::create_user_command::CreateUserCommand;
use crate::application::create_user::create_user_use_case::CreateUserUseCase;

pub struct CreateUserCommandHandler;

impl CreateUserCommandHandler {
    pub fn new() -> Self {
        return Self { };
    }
}

impl CreateUserUseCase for CreateUserCommandHandler{
    fn create(&self, create_user_command: CreateUserCommand) -> String {
        println!("UserCreated");
        return create_user_command.id;
    }
}
