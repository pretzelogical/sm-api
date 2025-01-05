use serde::{Serialize, Deserialize};
use sea_orm::entity::prelude::*;


#[derive(Serialize, Deserialize, Debug, Clone, DeriveEntityModel)]
#[sea_orm(table_name = "sm_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i64,
    pub name: String,
    pub pass: String,
}

// impl Default for Model {
//     fn default() -> Self {
//         Model {
//             id: -1,
//             name: "".to_string(),
//             pass: "".to_string()
//         }
//     }
// }

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// impl User {
//     pub async fn get(client: &Client, id: i64) -> Option<User> {
//         let _stmt = include_str!("../sql/user/get.sql");
//         let stmt = client.prepare(&_stmt).await.unwrap();

//         client
//             .query(&stmt, &[&id])
//             .await
//             .unwrap()
//             .iter()
//             .map(|row| User::from_row_ref(row).unwrap())
//             .collect::<Vec<User>>()
//             .pop()
//     }


//     pub async fn create(client: &Client, user_info: CreateUserArgs) -> Option<User> {
//         let _stmt = include_str!("../sql/user/create.sql");
//         let _stmt = client.prepare(&_stmt).await;
//         let stmt = match _stmt {
//             Ok(stmt) => stmt,
//             Err(_) => panic!("Cannot prepare create new user statement")
//         };

//         client
//             .query(
//                 &stmt,
//                 &[
//                     &user_info.name,
//                     &user_info.pass
//                 ]
//             )
//             .await
//             .expect("Could not create new user")
//             .iter()
//             .map(|row| User::from_row_ref(row).unwrap())
//             .collect::<Vec<User>>()
//             .pop()
//     }
// }