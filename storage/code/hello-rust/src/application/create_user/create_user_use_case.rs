use crate::application::create_user::create_user_command::CreateUserCommand;

pub trait CreateUserUseCase{
    fn create(&self,create_user_command: CreateUserCommand) -> String;
}