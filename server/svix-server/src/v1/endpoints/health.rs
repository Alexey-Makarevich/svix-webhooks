// SPDX-FileCopyrightText: © 2022 Svix Authors
// SPDX-License-Identifier: MIT

use std::time::Duration;

use aide::axum::ApiRouter;
use axum::{extract::State, http::StatusCode, routing::get, Json};
use sea_orm::{query::Statement, ConnectionTrait, DatabaseBackend};
use serde::{Deserialize, Serialize};

use crate::{
    core::cache::{kv_def, CacheBehavior, CacheKey, CacheValue},
    queue::QueueTask,
    AppState,
};


use crate::cfg::Configuration;
use lazy_static::lazy_static;


lazy_static! {

    static ref REDISPREFIX: String ={
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();
        let redis_prefix=cfg.redis_prefix.as_ref().unwrap(); //.to_owned();
        redis_prefix.to_owned()
    };

    // static ref MAIN: String = REDISPREFIX.as_str().to_owned() + &"{queue}_svix_v3_main".to_string();
    // static ref DELAYED: String = REDISPREFIX.as_str().to_owned() + &"{queue}_svix_delayed".to_string();
    // static ref DELAYED_LOCK: String = REDISPREFIX.as_str().to_owned() + &"{queue}_svix_delayed_lock".to_string();
    // static ref LEGACY_V2_MAIN: String = REDISPREFIX.as_str().to_owned() + &"{queue}_svix_main".to_string();
    // static ref LEGACY_V2_PROCESSING: String = REDISPREFIX.as_str().to_owned() + &"{queue}_svix_processing".to_string();
    // static ref LEGACY_V1_MAIN: String = REDISPREFIX.as_str().to_owned() + &"svix_queue_main".to_string();
    // static ref LEGACY_V1_PROCESSING: String = REDISPREFIX.as_str().to_owned() + &"svix_queue_processing".to_string();
    // static ref LEGACY_V1_DELAYED: String = REDISPREFIX.as_str().to_owned() + &"svix_queue_delayed".to_string();

}






async fn ping() -> StatusCode {
    StatusCode::NO_CONTENT
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatusVariant {
    Ok,
    Error,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthStatus {
    status: HealthStatusVariant,
    // TODO: information field
}

impl HealthStatus {
    pub fn new_ok() -> HealthStatus {
        HealthStatus {
            status: HealthStatusVariant::Ok,
        }
    }

    pub fn new_error() -> HealthStatus {
        HealthStatus {
            status: HealthStatusVariant::Error,
        }
    }

    pub fn is_ok(&self) -> bool {
        matches!(
            self,
            HealthStatus {
                status: HealthStatusVariant::Ok,
                ..
            }
        )
    }
}
impl<O, E> From<Result<O, E>> for HealthStatus {
    fn from(res: Result<O, E>) -> Self {
        match res {
            Ok(_) => HealthStatus::new_ok(),
            Err(_) => HealthStatus::new_error(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HealthReport {
    database: HealthStatus,

    queue: HealthStatus,
    cache: HealthStatus,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct HealthCheckCacheValue(());
kv_def!(HealthCheckCacheKey, HealthCheckCacheValue, "SVIX_CACHE"); // !!!!!!!!!!
impl HealthCheckCacheKey {
    fn new() -> HealthCheckCacheKey {
        HealthCheckCacheKey(format!("{}SVIX_CACHE", REDISPREFIX.as_str().to_owned()))
    }
}


async fn health(
    State(AppState {
        ref db,
        queue_tx,
        cache,
        ..
    }): State<AppState>,
) -> (StatusCode, Json<HealthReport>) {
    // SELECT 1 FROM any table
    let database: HealthStatus = db
        .execute(Statement::from_string(
            DatabaseBackend::MySql,
            "SELECT 1".to_owned(),
        ))
        .await
        .into();

    // Send a [`HealthCheck`] through the queue
    let queue: HealthStatus = queue_tx.send(QueueTask::HealthCheck, None).await.into();

    // Set a cache value with an expiration to ensure it works
    let cache: HealthStatus = cache
        .set(
            &HealthCheckCacheKey(format!("{}health_check_value", REDISPREFIX.as_str().to_owned())), // !!!!!!!!!!
            &HealthCheckCacheValue(()),
            // Expires after this time, so it won't pollute the DB
            Duration::from_millis(100),
        )
        .await
        .into();

    let status = if database.is_ok() && queue.is_ok() && cache.is_ok() {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };

    (
        status,
        Json(HealthReport {
            database,
            queue,
            cache,
        }),
    )
}

pub fn router() -> ApiRouter<AppState> {
    ApiRouter::new()
        .route("/health/ping/", get(ping).head(ping))
        .route("/health/", get(health).head(health))
}



#[cfg(test)]
mod tests {


    // testtttttt
    #[tokio::test]
    async fn test_set_read_key_with_prefix() {
        dotenv::dotenv().ok();
        let cfg = crate::cfg::load().unwrap();


        println!("Start test");

        println!("redis_dsn from cfg = {}", cfg.redis_dsn.as_ref().unwrap().as_str());

        println!("redis_prefix from cfg = {}", cfg.redis_prefix.as_ref().unwrap().as_str());


        println!("REDISPREFIX={}", super::REDISPREFIX.as_str());


        println!("End test");
    }



}