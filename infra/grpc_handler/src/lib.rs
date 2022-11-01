pub mod user {
    pub mod v1 {
        tonic::include_proto!("user.v1");
    }
}

mod convert;

use app_context::AppContext;
use derive_getters::Getters;
use derive_new::new;
use error::AppError;
use tonic::{Request, Response, Status};
use user::v1::user_service_server::UserService;
use user::v1::{
    CreateUserRequest, CreateUserResponse, GetUsersByIdsRequest, GetUsersByIdsResponse,
};

#[derive(new, Getters)]
pub struct UserServiceHandler {
    ctx: AppContext,
}

#[tonic::async_trait]
impl UserService for UserServiceHandler {
    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let cmd = request
            .into_inner()
            .try_into()
            .map_err(|e| handle_error(e))?;

        let user = usecase::create_user(self.ctx(), cmd)
            .await
            .map_err(|e| handle_error(e))?;

        Ok(Response::new(user.into()))
    }

    async fn get_users_by_ids(
        &self,
        request: Request<GetUsersByIdsRequest>,
    ) -> Result<Response<GetUsersByIdsResponse>, Status> {
        let ids = request
            .into_inner()
            .try_into()
            .map_err(|e| handle_error(e))?;

        let users = usecase::get_users_by_ids(self.ctx(), ids)
            .await
            .map_err(|e| handle_error(e))?;

        Ok(Response::new(users.into()))
    }
}

fn handle_error(err: anyhow::Error) -> Status {
    eprintln!("{err:?}");

    match err.downcast_ref::<AppError>() {
        Some(err) => match err {
            AppError::InvalidArgument(msg) => Status::invalid_argument(msg),
            AppError::NotFound(msg) => Status::not_found(msg),
            AppError::Internal(msg) => Status::internal(msg),
        },
        None => Status::internal("internal error"),
    }
}
