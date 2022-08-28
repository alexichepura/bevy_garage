// use crate::{
//     db::{post, user},
//     db_client::DbClientResource,
// };
// use bevy::prelude::*;

// #[tokio::main]
// pub async fn db_start_system(dbres: Res<DbClientResource>) {
//     let posts: Vec<post::Data> = dbres.client.post().find_many(vec![]).exec().await.unwrap();
//     let users: Vec<user::Data> = dbres.client.user().find_many(vec![]).exec().await.unwrap();

//     println!("posts {:?}", posts);
//     println!("users {:?}", users);

//     let created = dbres
//         .client
//         .user()
//         .create(user::display_name::set("what up".to_string()), vec![])
//         .exec()
//         .await;
//     println!("created {:?}", created);

//     let users: Vec<user::Data> = dbres.client.user().find_many(vec![]).exec().await.unwrap();
//     println!("users2 {:?}", users);
//     println!("users2 {:?}", users);
// }
