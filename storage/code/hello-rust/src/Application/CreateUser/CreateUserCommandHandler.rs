
struct CreateUserCommandHandler{

}

impl CreateUserUseCase for CreateUserCommandHandler{
    fn create() -> String{
        let user = User{
            id: String::from("id"),
            name: String::from("name"),
        };

    }
}