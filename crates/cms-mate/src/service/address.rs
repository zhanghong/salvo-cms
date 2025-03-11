use redis::{Client, ErrorKind, JsonAsyncCommands, RedisError, RedisResult};
use redis_macros::Json;
use serde::{Deserialize, Serialize};

/// Define structs to hold the data
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Address {
    Street(String),
    Road(String),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct User {
    id: u32,
    name: String,
    addresses: Vec<Address>,
}

pub struct AddressService {}

impl AddressService {
    pub async fn redis_store()  -> RedisResult<()> {
        // Open new connection to localhost
        let client = Client::open("redis://localhost:6379")?;
        let mut con = client.get_multiplexed_async_connection().await.map_err(|_| {
            RedisError::from((
                ErrorKind::InvalidClientConfig,
                "Cannot connect to localhost:6379. Try starting a redis-server process or container.",
            ))
        })?;
    
        // Define the data you want to store in Redis.
        let user = User {
            id: 1,
            name: "Ziggy".to_string(),
            addresses: vec![
                Address::Street("Downing".to_string()),
                Address::Road("Abbey".to_string()),
            ],
        };
    
        // Wrap the data in `Json(..)` when reading from from Redis
        let _: () = con.json_set("user_wrapped", "$", &user).await?;
        let Json(stored_user): Json<User> = con.json_get("user_wrapped", "$").await?;
        assert_eq!(user, stored_user);
    
        // You can unwrap inner structs as well
        let Json(stored_address): Json<Address> =
            con.json_get("user_wrapped", "$.addresses[0]").await?;
        assert_eq!(user.addresses[0], stored_address);
    
        // Even with types that redis normally overrides (e.g. String, Vec)
        let Json(stored_name): Json<String> = con.json_get("user_wrapped", "$.name").await?;
        assert_eq!(user.name, stored_name);
        let Json(stored_addresses): Json<Vec<Address>> =
            con.json_get("user_wrapped", "$.addresses").await?;
        println!("{:?}", stored_addresses);
        assert_eq!(user.addresses, stored_addresses);
    
        // You can even use these types as inputs
        let users = vec![user];
        let _: () = con.json_set("users_wrapped", "$", &users).await?;
        let Json(stored_users): Json<Vec<User>> = con.json_get("users_wrapped", "$").await?;
        assert_eq!(users, stored_users);
    
    
        Ok(())
    }
}