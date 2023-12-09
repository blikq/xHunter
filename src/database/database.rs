use redis::{Client, JsonAsyncCommands, ErrorKind, RedisError, RedisResult};
use redis_macros::{FromRedisValue, ToRedisArgs, Json};
use std::time::Instant;
use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Bullet {
    pub location: Vec<f64>,
    pub time: f64,
    pub direction: Direction,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Character{
    pub direction: Direction,
    pub location: (f64, f64),
    pub last_log: f64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, FromRedisValue, ToRedisArgs)]
pub struct Players{
    pub player_0: Character,
    pub player_1: Character,
    // pub bullets: Vec<Bullet>,
}

#[tokio::main]
pub async fn receive_log(idf: i32) -> redis::RedisResult<((Character))>{
    // Open a connection
    let client = redis::Client::open("redis://default:M50Ly0PDe8zSlnh0exRENf7jYYn5xNLn@redis-12336.c300.eu-central-1-1.ec2.cloud.redislabs.com:12336").unwrap();
    let mut con = client.get_async_connection().await.map_err(|_| {
        RedisError::from((
            ErrorKind::InvalidClientConfig,
            "Cannot connect to localhost:6379. Try starting a redis-server process or container.",
        ))
    })?;
    let stored_users: Players = con.json_get("test", "$").await?;
    let opponent:Character;
    // let bullets:Vec<Bullet>;
    if idf == 0 {
        opponent = stored_users.player_1;
        // bullets = stored_users.bullets;
    }else{
        opponent = stored_users.player_0;
        // bullets = stored_users.bullets;
    }
    let result = (opponent);
    // Define the data you want to store in Redis.
    // let user = User {
    //     location: (2.0, 4.0),
    //     last_upd: 0.3,
    // };
    // con.json_set("user_json", "$", &user).await?;
    // con.json_clear("user_json", "$");

    // println!("{:?}", stored_user);
    Ok(result)
}

#[tokio::main]
pub async fn send_log(idf: i32) -> redis::RedisResult<()>{
    // Open a connection
    let client = redis::Client::open("redis://default:M50Ly0PDe8zSlnh0exRENf7jYYn5xNLn@redis-12336.c300.eu-central-1-1.ec2.cloud.redislabs.com:12336").unwrap();
    let mut con = client.get_async_connection().await.map_err(|_| {
        RedisError::from((
            ErrorKind::InvalidClientConfig,
            "Cannot connect to localhost:6379. Try starting a redis-server process or container.",
        ))
    })?;
    // let user = Players {
    //     location: (2.0, 4.0),
    //     last_upd: 0.3,
    // };
    // if idf == 0 {

    // }
    Ok(())
    
}

// #[tokio::main]
// pub async fn setup_t() -> redis::RedisResult<()>{
//     // Open a connection
//     let client = redis::Client::open("redis://default:M50Ly0PDe8zSlnh0exRENf7jYYn5xNLn@redis-12336.c300.eu-central-1-1.ec2.cloud.redislabs.com:12336").unwrap();
//     let mut con = client.get_async_connection().await.map_err(|_| {
//         RedisError::from((
//             ErrorKind::InvalidClientConfig,
//             "Cannot connect to localhost:6379. Try starting a redis-server process or container.",
//         ))
//     })?;
//     let user = Players {
//         player_0: Character { direction: Direction::Up, location: (0.0, 0.0), last_log: 132324.0 },
//         player_1: Character { direction: Direction::Up, location: (0.0, 0.0), last_log: 132324.0 },
//     };

//     con.json_set("test", "$", &user).await?;
//     println!("mfing successfully");
//     Ok(())

// }

#[tokio::main]
pub async fn get_idf() -> redis::RedisResult<(i32)>{
    // Open a connection
    let client = redis::Client::open("redis://default:M50Ly0PDe8zSlnh0exRENf7jYYn5xNLn@redis-12336.c300.eu-central-1-1.ec2.cloud.redislabs.com:12336").unwrap();
    let mut con = client.get_async_connection().await.map_err(|_| {
        RedisError::from((
            ErrorKind::InvalidClientConfig,
            "Cannot connect to localhost:6379. Try starting a redis-server process or container.",
        ))
    })?;
    
    let stored_users: Players = con.json_get("test", "$").await?;
    let mut idf:i32 = 0;
    if stored_users.player_0.last_log > 6.0{
        idf = 0;
    };
    if stored_users.player_1.last_log > 6.0{
        idf = 1;
    };

    Ok((idf))

}