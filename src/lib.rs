//! Application identity and filesystem locations shared by Bottles Next crates.
//!
//! Every location is derived from [`APP_ID`] and resolved once per process
//! using the platform's conventions.

use directories::ProjectDirs;
use std::{
    path::{Path, PathBuf},
    sync::LazyLock,
};

/// Reverse-DNS application identifier, as `qualifier.organization.application`.
///
/// Forks should change this to keep their data separate from Bottles Next.
pub const APP_ID: &str = "com.usebottles.bottles-next";

static DIRECTORIES: LazyLock<ProjectDirs> = LazyLock::new(|| {
    let parts: Vec<&str> = APP_ID.splitn(3, '.').collect();
    ProjectDirs::from(parts[0], parts[1], parts[2])
        .expect("the platform does not provide a home directory")
});

/// Returns the directory for configuration files.
pub fn config_dir() -> &'static Path {
    DIRECTORIES.config_dir()
}

/// Returns the directory for persistent data local to this machine.
pub fn data_dir() -> &'static Path {
    DIRECTORIES.data_local_dir()
}

/// Returns the directory for data that may be deleted and recreated.
pub fn cache_dir() -> &'static Path {
    DIRECTORIES.cache_dir()
}

/// Returns the directory for sockets and other per-session files.
///
/// Platforms without a runtime directory use `runtime` below [`data_dir`].
pub fn runtime_dir() -> &'static Path {
    static RUNTIME: LazyLock<PathBuf> = LazyLock::new(|| {
        DIRECTORIES
            .runtime_dir()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| data_dir().join("runtime"))
    });
    &RUNTIME
}

/// Returns the directory containing installed plugin packages.
pub fn plugins_dir() -> &'static Path {
    static PLUGINS: LazyLock<PathBuf> = LazyLock::new(|| data_dir().join("plugins"));
    &PLUGINS
}
