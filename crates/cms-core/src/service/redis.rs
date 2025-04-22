use redis::JsonAsyncCommands;
use redis::{
    Client, Commands, Connection, FromRedisValue, ToRedisArgs, aio::MultiplexedConnection,
};
use redis_macros::Json;
use serde::{Deserialize, Serialize};
use std::marker::{Send, Sync};

use crate::utils::time;

pub struct RedisService {}

impl RedisService {
    fn get_connection(client: &Client) -> Connection {
        client.get_connection().unwrap()
    }

    async fn get_async_connection(client: &Client) -> MultiplexedConnection {
        client.get_multiplexed_async_connection().await.unwrap()
    }

    pub fn set<K: ToRedisArgs, V: ToRedisArgs>(client: &Client, key: K, value: V) -> () {
        let mut con = Self::get_connection(client);
        let _: () = con.set(key, value).unwrap();
    }

    pub fn set_ex<K: ToRedisArgs, V: ToRedisArgs>(
        client: &Client,
        key: K,
        value: V,
        ex: u64,
    ) -> () {
        let mut con = Self::get_connection(client);
        let _: () = con.set_ex(key, value, ex).unwrap();
    }

    pub fn get<K: ToRedisArgs, T: FromRedisValue>(client: &Client, key: K) -> Option<T> {
        let mut con = Self::get_connection(client);
        con.get(key).unwrap()
    }

    pub fn del<K: ToRedisArgs>(client: &Client, key: K) -> () {
        let mut con = Self::get_connection(client);
        let _: () = con.del(key).unwrap();
    }

    pub fn set_jwt_key(client: &Client, jwt_id: &String, expired_time: i64) {
        let current_timestamp = time::current_timestamp();
        let mut seconds: i64 = expired_time - current_timestamp;
        if seconds < 0 {
            seconds = 1;
        }
        Self::set_ex(client, Self::to_jwt_key(jwt_id), true, seconds as u64);
    }

    pub fn has_jwt_key(client: &Client, jwt_id: &String) -> bool {
        let value: bool = Self::get(client, Self::to_jwt_key(jwt_id)).unwrap_or(false);
        value
    }

    pub fn del_jwt_key(client: &Client, jwt_id: &String) {
        Self::del(client, Self::to_jwt_key(jwt_id));
    }

    fn to_jwt_key(id: &String) -> String {
        format!("jwt:{}", id)
    }

    pub async fn set_json_list<T: Serialize + Send + Sync>(
        client: &Client,
        key: &str,
        list: &Vec<T>,
    ) {
        let mut con = Self::get_async_connection(client).await;
        let _: () = con.json_set(key, "$", list).await.unwrap();
    }

    pub async fn get_json_list<T: for<'a> Deserialize<'a>>(client: &Client, key: &str) -> Vec<T> {
        let mut con = Self::get_async_connection(client).await;
        let stored_list: Vec<T> = match con.json_get(key, "$").await {
            Ok(Json(list)) => list,
            Err(_) => Vec::new(),
        };

        stored_list
    }
}
