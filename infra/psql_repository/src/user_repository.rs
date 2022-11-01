use anyhow::{self, Context};
use async_trait::async_trait;
use derive_new::new;
use diesel::{
    pg::{upsert::excluded, PgConnection},
    prelude::*,
    r2d2::ConnectionManager,
    Insertable, Queryable,
};
use r2d2::Pool;

use db_schema::users;
use error::AppError;

#[derive(Queryable, Insertable)]
#[table_name = "users"]
struct User {
    pub id: String,
    pub name: String,
}

impl From<&entity::User> for User {
    fn from(user: &entity::User) -> User {
        User {
            id: user.id().to_string(),
            name: user.name().to_string(),
        }
    }
}

impl TryFrom<User> for entity::User {
    type Error = anyhow::Error;

    fn try_from(user: User) -> Result<Self, Self::Error> {
        let User { id, name } = user;

        entity::User::reconstruct(id, name)
    }
}

#[derive(new)]
pub struct UserRepository {
    pool: Pool<ConnectionManager<PgConnection>>,
}

#[async_trait]
impl repository::UserRepository for UserRepository {
    async fn save(&self, user: &entity::User) -> anyhow::Result<()> {
        tokio::task::block_in_place(|| {
            let user = User::from(user);
            let conn = self.pool.get()?;

            diesel::insert_into(users::table)
                .values(user)
                .on_conflict(users::id)
                .do_update()
                .set(users::name.eq(excluded(users::name)))
                .execute(&conn)
                .with_context(|| {
                    AppError::Internal("failed to insert or update user".to_string())
                })?;
            Ok(())
        })
    }

    async fn get_by_ids(&self, _ids: &[entity::UserId]) -> anyhow::Result<Vec<entity::User>> {
        todo!()
        // TODO: load でエラー
        // tokio::task::block_in_place(|| {
        //     let ids = ids.iter().map(|id| id.to_string()).collect::<Vec<_>>();
        //     let conn = self.pool.get()?;

        //     let users = users::table
        //         .filter(users::id.eq_any(ids))
        //         .load::<User>(&conn)
        //         .with_context(|| AppError::Internal("failed to get user".to_string()))?;

        //     users.into_iter().map(TryInto::try_into).collect()
        // })
    }
}
