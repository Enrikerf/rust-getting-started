use crate::application::create_user::create_user_command::CreateUserCommand;
use crate::Id;

pub trait CreateUserUseCase{
    fn create(&self,create_user_command: CreateUserCommand) -> Id;
}