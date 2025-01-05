use deadpool_postgres::Client;
use serde::{Serialize, Deserialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::routes::CreateUserArgs;


#[derive(Serialize, Deserialize, PostgresMapper, Debug)]
#[pg_mapper(table = "sm_user")]
pub struct User {
    pub id: i64,
    pub name: String,
    pub pass: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: -1,
            name: "".to_string(),
            pass: "".to_string()
        }
    }
}

impl User {
    pub async fn get(client: &Client, id: i64) -> Option<User> {
        let _stmt = include_str!("../sql/user/get.sql");
        let stmt = client.prepare(&_stmt).await.unwrap();

        client
            .query(&stmt, &[&id])
            .await
            .unwrap()
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop()
    }


    pub async fn create(client: &Client, user_info: CreateUserArgs) -> Option<User> {
        let _stmt = include_str!("../sql/user/create.sql");
        let _stmt = client.prepare(&_stmt).await;
        let stmt = match _stmt {
            Ok(stmt) => stmt,
            Err(_) => panic!("Cannot prepare create new user statement")
        };

        client
            .query(
                &stmt,
                &[
                    &user_info.name,
                    &user_info.pass
                ]
            )
            .await
            .expect("Could not create new user")
            .iter()
            .map(|row| User::from_row_ref(row).unwrap())
            .collect::<Vec<User>>()
            .pop()
    }
}