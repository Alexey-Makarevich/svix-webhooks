use std::ops::{Deref, DerefMut};

// use axum::async_trait;
use bb8::PooledConnection;

use redis::{ErrorKind, IntoConnectionInfo, RedisError};
use redis_cluster_async::Client;

use async_trait::async_trait;

/// ConnectionManager that implements `bb8::ManageConnection` and supports
/// asynchronous clustered connections via `redis_cluster_async::Connection`
#[derive(Clone)]
pub struct RedisClusterConnectionManager {
    client: Client,
}

impl RedisClusterConnectionManager {
    pub fn new<T: IntoConnectionInfo>(
        initial_nodes: Vec<T>,
    ) -> Result<RedisClusterConnectionManager, RedisError> {
        Ok(RedisClusterConnectionManager {
            client: Client::open(initial_nodes)?,

            // test connection to redis cluster with ACL
            // client: Client::open(vec!["redis://svix:svixpass@redis-cluster:6379/", "redis://svix:svixpass@redis-cluster-node-0:6379/", "redis://svix:svixpass@redis-cluster-node-1:6379/", "redis://svix:svixpass@redis-cluster-node-2:6379/", "redis://svix:svixpass@redis-cluster-node-3:6379/", "redis://svix:svixpass@redis-cluster-node-4:6379/"])?,

        })
    }
}

#[async_trait]
impl bb8::ManageConnection for RedisClusterConnectionManager {
    type Connection = redis_cluster_async::Connection;
    type Error = RedisError;

    // async fn connect(&self) -> Result<Self::Connection, Self::Error> {
    //     self.client.get_connection().await
    // }

    async fn connect(&self) -> Result<Self::Connection, Self::Error> {
        self.client.get_connection().await.into()
    }

    async fn is_valid(&self, conn: &mut Self::Connection) -> Result<(), Self::Error> {
        let pong: String = redis::cmd("PING").query_async(&mut *conn).await?;
        match pong.as_str() {
            "PONG" => Ok(()),
            _ => Err((ErrorKind::ResponseError, "ping request").into()),
        }
    }


    // async fn is_valid(&self, conn: &mut bb8::PooledConnection<'_, Self>, ) -> Result<(), Self::Error> {
    //     let pong: String = redis::cmd("PING").query_async(conn.deref_mut()).await?;
    //     match pong.as_str() {
    //         "PONG" => Ok(()),
    //         _ => Err((ErrorKind::ResponseError, "ping request").into()),
    //     }
    // }





    fn has_broken(&self, _: &mut Self::Connection) -> bool {
        false
    }
}

