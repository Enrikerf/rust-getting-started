use crate::application::create_user::{create_user_command::CreateUserCommand, create_user_command_handler::CreateUserCommandHandler};
use crate::infrastructure::create_user::create_user_controller::CreateUserController;

pub struct Server;

impl Server {
    pub fn new() -> Self {
        return Self {};
    }
    pub fn use_app(&self) {
        let command = CreateUserCommand {
            id: "id".to_string(),
            name: "name".to_string(),
        };
        let command_handler = CreateUserCommandHandler {};
        let controller = CreateUserController { create_user_use_case: command_handler };
        controller.create(command)
    }
}
