// use redis::{Client, JsonAsyncCommands, ErrorKind, RedisError, RedisResult};
// use redis_macros::{FromRedisValue, ToRedisArgs, Json};
// use std::time::Instant;
// use serde_json;
// use serde::{Deserialize, Serialize};

// #[derive(Debug, PartialEq, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
// pub struct User{
//     location: (f64, f64),
//     last_upd: f64,
// }

// #[tokio::main]
// pub async fn conn() -> redis::RedisResult<()>{
//     // Open a connection
//     let client = redis::Client::open("redis://default:M50Ly0PDe8zSlnh0exRENf7jYYn5xNLn@redis-12336.c300.eu-central-1-1.ec2.cloud.redislabs.com:12336").unwrap();
//     let mut con = client.get_async_connection().await.map_err(|_| {
//         RedisError::from((
//             ErrorKind::InvalidClientConfig,
//             "Cannot connect to localhost:6379. Try starting a redis-server process or container.",
//         ))
//     })?;
//     // Define the data you want to store in Redis.
//     let user = User {
//         location: (2.0, 4.0),
//         last_upd: 0.3,
//     };
//     // con.json_set("user_json", "$", &user).await?;
//     con.json_clear("user_json", "$");
//     let stored_user: User = con.json_get("user_json", "$").await?;
//     println!("{:?}", stored_user);
//     Ok(())
// }
