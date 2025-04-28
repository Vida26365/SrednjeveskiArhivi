use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::LazyLock;

use cfg_if::cfg_if;
use dioxus::logger::tracing::info;
use directories::BaseDirs;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppDirectories {
    pub sysdata: PathBuf,
    pub userdata: PathBuf,
}

impl AppDirectories {
    pub fn new() -> Self {
        info!("Determining application directories...");

        let sysdata = PathBuf::from("."); // TODO: Set the correct path on all platforms

        let userdata = {
            let base = BaseDirs::new().expect("Failed to determine base system directories");
            let data = base.data_dir();

            cfg_if! {
                if #[cfg(any(platform_windows, platform_macos))] {
                    data.join("SrednjeveskiArhivi")
                } else if #[cfg(any(platform_linux, platform_bsd))] {
                    data.join("srednjeveski-arhivi")
                } else {
                    compile_error!("Unknown operating system")
                }
            }
        };

        info!("System data directory: {}", sysdata.display());
        info!("User data directory: {}", userdata.display());

        info!("Creating application directories...");
        create_dir_all(&userdata).expect("Failed to create user data directory");

        Self { sysdata, userdata }
    }
}

pub static DIRECTORIES: LazyLock<AppDirectories> = LazyLock::new(AppDirectories::new);
