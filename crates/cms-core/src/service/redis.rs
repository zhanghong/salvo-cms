use redis::{Client, Commands, Connection, FromRedisValue, ToRedisArgs};

pub struct RedisService {}

impl RedisService {
    fn get_connection(client: &Client) -> Connection {
        client.get_connection().unwrap()
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
}
