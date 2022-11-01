use psql_repository::UserRepository;
use repository::ProvideUserRepository;

pub struct AppContext {
    pub user_repository: UserRepository,
}

impl ProvideUserRepository for AppContext {
    type Repository = UserRepository;

    fn provide(&self) -> &Self::Repository {
        &self.user_repository
    }
}
