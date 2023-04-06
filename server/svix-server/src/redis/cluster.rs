use std::ops::DerefMut;

use axum::async_trait;

use redis::{ErrorKind, IntoConnectionInfo, RedisError};
use redis_cluster_async::Client;

/// ConnectionManager that implements `bb8::ManageConnection` and supports
/// asynchronous clustered connections via `redis_cluster_async::Connection`
#[derive(Clone)]
pub struct RedisClusterConnectionManager {
    client: Client,
}

impl RedisClusterConnectionManager {
    pub fn new<T: IntoConnectionInfo>(
        info: T,
    ) -> Result<RedisClusterConnectionManager, RedisError> {
        Ok(RedisClusterConnectionManager {
            // client: Client::open(vec![info])?,

            // # SVIX_REDIS_DSN: "redis://redis-cluster:6379/,redis://redis-cluster-node-0:6379/,redis://redis-cluster-node-1:6379/,redis://redis-cluster-node-2:6379/,redis://redis-cluster-node-3:6379/,redis://redis-cluster-node-4:6379/"

            client: Client::open(vec!["redis://redis-cluster:6379/", "redis://redis-cluster-node-0:6379/", "redis://redis-cluster-node-1:6379/", "redis://redis-cluster-node-2:6379/", "redis://redis-cluster-node-3:6379/", "redis://redis-cluster-node-4:6379/"])?,
        })
    }
}

#[async_trait]
impl bb8::ManageConnection for RedisClusterConnectionManager {
    type Connection = redis_cluster_async::Connection;
    type Error = RedisError;

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_connection().await
    }

    async fn is_valid(
        &self,
        conn: &mut bb8::PooledConnection<'_, Self>,
    ) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(conn.deref_mut()).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }

    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

// see https://github.com/svix/redis-cluster-async
// fn client_open_test() {
//     let nodes = vec!["redis://127.0.0.1:6379/", "redis://127.0.0.1:6378/", "redis://127.0.0.1:6377/"];
//
//     let mut runtime = tokio::runtime::Runtime::new().unwrap();
//
//     let client = Client::open(nodes).unwrap();
// }


