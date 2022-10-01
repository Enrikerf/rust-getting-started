use std::fmt::Error;

use uuid::Uuid;

use crate::{Id, Name};
use crate::application::create_user::create_user_command::CreateUserCommand;
use crate::application::create_user::create_user_use_case::CreateUserUseCase;

pub struct CreateUserController<T: CreateUserUseCase> {
    pub create_user_use_case: T,
}

impl<T: CreateUserUseCase> CreateUserController<T> {
    pub fn create(&self) -> Result<Uuid, Error> {
        let uuid_parse_result = Uuid::parse_str("");
        let uuid = match uuid_parse_result {
            Ok(uuid) => uuid,
            Err(error) => return Err(Default::default()),
        };
        let command = CreateUserCommand {
            id: Id { value: uuid },
            name: Name { value: "name".to_string() },
        };
        &self.create_user_use_case.create(command);
        return Ok(uuid);
    }
}
