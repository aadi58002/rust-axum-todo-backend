use axum::Extension;
use core::{creation::create_table, User, deletion::drop_table};

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
) -> String {
    create_table(&db_connection, User::Entity).await;
    drop_table(&db_connection, User::Entity).await;
    "Success".to_string()
}
