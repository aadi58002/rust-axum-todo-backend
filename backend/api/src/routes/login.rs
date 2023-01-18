use axum::Extension;
use axum::response::IntoResponse;
use core::Tasks::TaskState;
use core::creation::create_type;
use core::deletion::drop_type;
use core::sea_orm::entity::*;
use core::{
    clear::clear_table, creation::create_table, deletion::drop_table, insert::insert_in_table,
    Tasks, User,
};

// #[derive(Deserialize, Serialize,Debug)]
// pub struct User {
//     message: String,
// }

// Function to get the Json Body
// pub async fn login(Json(body): Json<User>) -> Json<User>{
//     Json(body)
// }

// Function to get the path variables from the route
// pub async fn login(Path(id): Path<i32>) -> String{
//     id.to_string()
// }

// #[derive(Deserialize,Serialize,Debug)]
// pub struct QueryValues{
//     message: String,
// }

// Function to get the Query parems from the route path
// pub async fn login(Query(values): Query<QueryValues>) -> String{
//     values.message
// }

// Function to get the headers from the request
// pub async fn login(headers: HeaderMap,body: String) -> (HeaderMap,String){
//     println!("{:?}",headers);
//     (headers,body)
// }

pub async fn login(
    Extension(db_connection): Extension<core::sea_orm::DatabaseConnection>,
) -> impl IntoResponse{
    create_table(&db_connection, User::Entity).await;
    create_type::<TaskState>(&db_connection).await;
    create_table(&db_connection, Tasks::Entity).await;
    let test = User::ActiveModel{
        username: Set("Test".to_owned()),
        password: Set("Test".to_owned()),
        email: Set("Test@gmail.com".to_owned()),
        ..Default::default()
    };
    match test.insert(&db_connection).await{
        Ok(v) => println!("Added User : {:?}", v),
        Err(e) => println!("{:?}",e),
    }
    "Success".to_string()
}
