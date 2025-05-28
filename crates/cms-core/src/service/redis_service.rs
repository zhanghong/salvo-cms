use redis::JsonAsyncCommands;
use redis::{
    Client, Commands, Connection, FromRedisValue, ToRedisArgs, aio::MultiplexedConnection,
};
use redis_macros::Json;
use serde::{Deserialize, Serialize};
use std::marker::{Send, Sync};

use crate::utils::time_utils;

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
        let current_timestamp = time_utils::current_timestamp();
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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::fixture::config::app::FakerAppState;

    // 初始化 Redis 客户端
    async fn setup_redis() -> Client {
        let state = FakerAppState::init();
        state.redis.clone()
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct TestItem {
        id: i32,
        name: String,
    }

    // 测试 set 和 get
    #[tokio::test]
    async fn test_set_get_and_del() {
        let client = setup_redis().await;
        let key = "test_key";
        let value = "test_value";

        RedisService::set(&client, key, value);
        let result: Option<String> = RedisService::get(&client, key);
        assert_eq!(result, Some(value.to_string()));

        RedisService::del(&client, key);
        let result_after_del: Option<String> = RedisService::get(&client, key);
        assert_eq!(result_after_del, None::<String>);
    }

    // 测试 set_ex
    #[tokio::test]
    async fn test_ex_set_and_get() {
        let client = setup_redis().await;
        let key = "test_ex_key";
        let value = "ex_value";

        RedisService::set_ex(&client, key, value, 1); // 1秒过期
        let result: Option<String> = RedisService::get(&client, key);
        assert_eq!(result, Some(value.to_string()));

        std::thread::sleep(Duration::from_secs(2));
        let result_after_expire: Option<String> = RedisService::get(&client, key);
        assert_eq!(result_after_expire, None);
    }

    // 测试 JWT key 设置
    #[tokio::test]
    async fn test_set_has_del_jwt_key() {
        let client = setup_redis().await;
        let jwt_id = "abc123".to_string();

        // 当前时间 +5 秒过期
        let current = time_utils::current_timestamp();
        RedisService::set_jwt_key(&client, &jwt_id, current + 5);

        assert!(RedisService::has_jwt_key(&client, &jwt_id));

        RedisService::del_jwt_key(&client, &jwt_id);
        assert!(!RedisService::has_jwt_key(&client, &jwt_id));
    }

    // 测试过期时间小于当前时间的情况
    #[tokio::test]
    async fn test_set_jwt_key_expired() {
        let client = setup_redis().await;
        let jwt_id = "expired_jwt".to_string();

        let past_time = time_utils::current_timestamp() - 10; // 已过期
        RedisService::set_jwt_key(&client, &jwt_id, past_time);

        // 应该设置为 1 秒有效期
        assert!(RedisService::has_jwt_key(&client, &jwt_id));
    }

    // 测试 JSON 列表的异步写入和读取
    #[tokio::test]
    async fn test_set_get_json_list() {
        let client = setup_redis().await;
        let key = "json_list_key";

        let items = vec![
            TestItem {
                id: 1,
                name: "Alice".to_string(),
            },
            TestItem {
                id: 2,
                name: "Bob".to_string(),
            },
        ];

        RedisService::set_json_list(&client, key, &items).await;

        let retrieved: Vec<TestItem> = RedisService::get_json_list(&client, key).await;
        assert_eq!(retrieved, items);
    }
}
