//! # Feature flags
//!
//! Tpu-client-next supports three features:
//!
//! - **`metrics`**: Enables implementation of the method `report_to_influxdb` for
//!   [`SendTransactionStats`] structure.
//! - **`log`**: Enables logging using `log` crate. It is enabled by default.
//! - **`tracing`**: Enables logging using `tracing` crate instead of `log`. This feature is
//!   mutually exclusive with `log`.

pub(crate) mod connection_worker;
pub mod connection_workers_scheduler;
pub mod send_transaction_stats;
pub mod workers_cache;
pub use crate::{
    connection_workers_scheduler::{ConnectionWorkersScheduler, ConnectionWorkersSchedulerError},
    send_transaction_stats::SendTransactionStats,
};
pub(crate) mod quic_networking;
pub(crate) use crate::quic_networking::QuicError;
pub mod leader_updater;
pub mod transaction_batch;

#[cfg(feature = "metrics")]
pub mod metrics;

// Logging abstraction module
pub(crate) mod logging;
