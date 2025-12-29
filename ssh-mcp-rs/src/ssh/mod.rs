//! SSH connection management module
//!
//! This module provides persistent SSH connection handling with automatic
//! reconnection, authentication, and session management.

pub mod config;
pub mod connection;
pub mod handler;

// Re-exports
pub use config::SshConfig;
pub use connection::SshConnectionManager;
pub use handler::SshHandler;
