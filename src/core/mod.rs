mod init;

#[cfg(feature = "server")]
pub use init::load_env;
