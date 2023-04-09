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

            // 20230405-svix-redis-cluster-backend-1  | + [ ! -z true ]
            // 20230405-svix-redis-cluster-backend-1  | + WAIT_FOR_ARG=--wait-for 15
            // 20230405-svix-redis-cluster-backend-1  | + exec svix-server --run-migrations --wait-for 15
            // 20230405-svix-redis-cluster-backend-1  | 2023-04-06T13:39:23.526498Z DEBUG svix_server: Migrations: Running
            // 20230405-svix-redis-cluster-backend-1  | 2023-04-06T13:39:23.537102Z DEBUG svix_server: Migrations: Success
            // 20230405-svix-redis-cluster-backend-1  | 2023-04-06T13:39:23.537272Z DEBUG app_start: svix_server: DB: Initializing pool
            // 20230405-svix-redis-cluster-backend-1  | 2023-04-06T13:39:23.539270Z DEBUG app_start: svix_server: DB: Started
            // 20230405-svix-redis-cluster-backend-1  | 2023-04-06T13:39:23.539288Z DEBUG app_start: svix_server: Cache: Initializing RedisCluster
            // 20230405-svix-redis-cluster-backend-1  | thread 'main' panicked at 'Error initializing redis cluster client: Invalid database number', /app/svix-server/src/redis/mod.rs:264:14
            // 20230405-svix-redis-cluster-backend-1  | stack backtrace:
            // 20230405-svix-redis-cluster-backend-1  |    0:     0x555ec0bb91a0 - std::backtrace_rs::backtrace::libunwind::trace::h32eb3e08e874dd27
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
            // 20230405-svix-redis-cluster-backend-1  |    1:     0x555ec0bb91a0 - std::backtrace_rs::backtrace::trace_unsynchronized::haa3f451d27bc11a5
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
            // 20230405-svix-redis-cluster-backend-1  |    2:     0x555ec0bb91a0 - std::sys_common::backtrace::_print_fmt::h5b94a01bb4289bb5
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:66:5
            // 20230405-svix-redis-cluster-backend-1  |    3:     0x555ec0bb91a0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb070b7fa7e3175df
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:45:22
            // 20230405-svix-redis-cluster-backend-1  |    4:     0x555ec0bddb3e - core::fmt::write::hd5207aebbb9a86e9
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/fmt/mod.rs:1202:17
            // 20230405-svix-redis-cluster-backend-1  |    5:     0x555ec0bb3065 - std::io::Write::write_fmt::h3bd699bbd129ab8a
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/io/mod.rs:1679:15
            // 20230405-svix-redis-cluster-backend-1  |    6:     0x555ec0bba9a3 - std::sys_common::backtrace::_print::h7a21be552fdf58da
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:48:5
            // 20230405-svix-redis-cluster-backend-1  |    7:     0x555ec0bba9a3 - std::sys_common::backtrace::print::ha85c41fe4dd80b13
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:35:9
            // 20230405-svix-redis-cluster-backend-1  |    8:     0x555ec0bba9a3 - std::panicking::default_hook::{{closure}}::h04cca40023d0eeca
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:295:22
            // 20230405-svix-redis-cluster-backend-1  |    9:     0x555ec0bba68f - std::panicking::default_hook::haa3ca8c310ed5402
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:314:9
            // 20230405-svix-redis-cluster-backend-1  |   10:     0x555ec0bbb04a - std::panicking::rust_panic_with_hook::h7b190ce1a948faac
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:698:17
            // 20230405-svix-redis-cluster-backend-1  |   11:     0x555ec0bbaf47 - std::panicking::begin_panic_handler::{{closure}}::hbafbfdc3e1b97f68
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:588:13
            // 20230405-svix-redis-cluster-backend-1  |   12:     0x555ec0bb964c - std::sys_common::backtrace::__rust_end_short_backtrace::hda93e5fef243b4c0
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/sys_common/backtrace.rs:138:18
            // 20230405-svix-redis-cluster-backend-1  |   13:     0x555ec0bbac62 - rust_begin_unwind
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:584:5
            // 20230405-svix-redis-cluster-backend-1  |   14:     0x555ebfb6c9b3 - core::panicking::panic_fmt::h8d17ca1073d9a733
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/panicking.rs:142:14
            // 20230405-svix-redis-cluster-backend-1  |   15:     0x555ebfb6cb03 - core::result::unwrap_failed::hfaddf24b248137d3
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/result.rs:1785:5
            // 20230405-svix-redis-cluster-backend-1  |   16:     0x555ebfd1cda2 - <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h92d2d2ceb0f296f2
            // 20230405-svix-redis-cluster-backend-1  |   17:     0x555ebfd1ee59 - <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::h9735a477738cdb4f
            // 20230405-svix-redis-cluster-backend-1  |   18:     0x555ebfcb8240 - svix_server::run_with_prefix::{{closure}}::h93798814f1579503
            // 20230405-svix-redis-cluster-backend-1  |   19:     0x555ebfd30b0c - <core::future::from_generator::GenFuture<T> as core::future::future::Future>::poll::hd5b280474ae28a26
            // 20230405-svix-redis-cluster-backend-1  |   20:     0x555ebfc654b7 - <tracing::instrument::Instrumented<T> as core::future::future::Future>::poll::h61f19aef6b55817f
            // 20230405-svix-redis-cluster-backend-1  |   21:     0x555ebfd3dfff - svix_server::main::{{closure}}::he2e2a0fac019ed58
            // 20230405-svix-redis-cluster-backend-1  |   22:     0x555ebfd86c65 - tokio::runtime::park::CachedParkThread::block_on::h0f084513c1b53f76
            // 20230405-svix-redis-cluster-backend-1  |   23:     0x555ebfbe1ea9 - tokio::runtime::scheduler::multi_thread::MultiThread::block_on::h6e48b6236b1ecfa1
            // 20230405-svix-redis-cluster-backend-1  |   24:     0x555ebfc358fa - tokio::runtime::runtime::Runtime::block_on::hb763389b62caa751
            // 20230405-svix-redis-cluster-backend-1  |   25:     0x555ebfdb0bac - svix_server::main::ha85e46d48d69e50c
            // 20230405-svix-redis-cluster-backend-1  |   26:     0x555ebfbb01d3 - std::sys_common::backtrace::__rust_begin_short_backtrace::hbfd3159aba322552
            // 20230405-svix-redis-cluster-backend-1  |   27:     0x555ebfc23029 - std::rt::lang_start::{{closure}}::hd07ba1c2e06d4858
            // 20230405-svix-redis-cluster-backend-1  |   28:     0x555ec0badc7f - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hb69be6e0857c6cfb
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/core/src/ops/function.rs:283:13
            // 20230405-svix-redis-cluster-backend-1  |   29:     0x555ec0badc7f - std::panicking::try::do_call::h396dfc441ee9c786
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:492:40
            // 20230405-svix-redis-cluster-backend-1  |   30:     0x555ec0badc7f - std::panicking::try::h6cdda972d28b3a4f
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:456:19
            // 20230405-svix-redis-cluster-backend-1  |   31:     0x555ec0badc7f - std::panic::catch_unwind::h376039ec264e8ef9
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panic.rs:137:14
            // 20230405-svix-redis-cluster-backend-1  |   32:     0x555ec0badc7f - std::rt::lang_start_internal::{{closure}}::hc94720ca3d4cb727
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/rt.rs:148:48
            // 20230405-svix-redis-cluster-backend-1  |   33:     0x555ec0badc7f - std::panicking::try::do_call::h2422fb95933fa2d5
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:492:40
            // 20230405-svix-redis-cluster-backend-1  |   34:     0x555ec0badc7f - std::panicking::try::h488286b5ec8333ff
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panicking.rs:456:19
            // 20230405-svix-redis-cluster-backend-1  |   35:     0x555ec0badc7f - std::panic::catch_unwind::h81636549836d2a25
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/panic.rs:137:14
            // 20230405-svix-redis-cluster-backend-1  |   36:     0x555ec0badc7f - std::rt::lang_start_internal::h6ba1bb743c1e9df9
            // 20230405-svix-redis-cluster-backend-1  |                                at /rustc/897e37553bba8b42751c67658967889d11ecd120/library/std/src/rt.rs:148:20
            // 20230405-svix-redis-cluster-backend-1  |   37:     0x555ebfdb0c98 - main
            // 20230405-svix-redis-cluster-backend-1  |   38:     0x7f8649ff6d0a - __libc_start_main
            // 20230405-svix-redis-cluster-backend-1  |   39:     0x555ebfb6cc9a - _start
            // 20230405-svix-redis-cluster-backend-1  |   40:                0x0 - <unknown>
            // 20230405-svix-redis-cluster-backend-1 exited with code 101


            // # SVIX_REDIS_DSN: "redis://redis-cluster:6379/,redis://redis-cluster-node-0:6379/,redis://redis-cluster-node-1:6379/,redis://redis-cluster-node-2:6379/,redis://redis-cluster-node-3:6379/,redis://redis-cluster-node-4:6379/"

            // client: Client::open(vec!["redis://redis-cluster:6379/", "redis://redis-cluster-node-0:6379/", "redis://redis-cluster-node-1:6379/", "redis://redis-cluster-node-2:6379/", "redis://redis-cluster-node-3:6379/", "redis://redis-cluster-node-4:6379/"])?,
            // with this variant all is working

            // added split in redis/mod.rs:264
            client: Client::open(vec![info])?,

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


