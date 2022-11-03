use diesel::r2d2::ConnectionManager;
use tonic::transport::Server;

use app_context::AppContext;
use grpc_handler::{user::v1::user_service_server::UserServiceServer, UserServiceHandler};
use psql_repository::UserRepository;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url =
        std::env::var("DATABASE_URL").expect("failed to read the env var DATABASE_URL");
    let manager = ConnectionManager::new(database_url);
    let pool = r2d2::Pool::new(manager).expect("failed to create the connection pool");

    let user_repository = UserRepository::new(pool);

    let context = AppContext { user_repository };

    let user_service = UserServiceHandler::new(context);

    let add = "127.0.0.1:50051".parse()?;
    println!("Start sample app server!");

    Server::builder()
        .add_service(UserServiceServer::new(user_service))
        .serve(add)
        .await?;

    Ok(())
}
