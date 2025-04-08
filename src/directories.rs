use std::fs::create_dir_all;
use std::path::PathBuf;
use std::sync::LazyLock;

use cfg_if::cfg_if;
use directories::BaseDirs;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AppDirectories {
    pub sysdata: PathBuf,
    pub userdata: PathBuf,
}

impl AppDirectories {
    pub fn new() -> Self {
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

        create_dir_all(&sysdata).expect("Failed to create system data directory");
        create_dir_all(&userdata).expect("Failed to create user data directory");

        Self { sysdata, userdata }
    }
}

pub static DIRECTORIES: LazyLock<AppDirectories> = LazyLock::new(AppDirectories::new);
