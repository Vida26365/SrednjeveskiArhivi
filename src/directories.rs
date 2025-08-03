use std::fs::create_dir_all;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use cfg_if::cfg_if;
use dioxus::logger::tracing::info;
use directories::BaseDirs;

trait PathClean {
    /// Cleans the path by removing redundant components such as `.` (current directory)
    /// and `..` (parent directory) while preserving the path target.
    fn clean(&self) -> PathBuf;
}

impl PathClean for Path {
    fn clean(&self) -> PathBuf {
        let mut components = self.components().peekable();
        let mut result = PathBuf::new();

        while let Some(component) = components.next() {
            match component {
                std::path::Component::CurDir => {}
                std::path::Component::ParentDir => {
                    result.pop();
                }
                _ => {
                    result.push(component.as_os_str());
                }
            }
        }

        result
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppDirectories {
    pub sysdata: PathBuf,
    pub userdata: PathBuf,
}

impl AppDirectories {
    pub fn new() -> Self {
        info!("Determining application directories...");

        let sysdata = {
            let executable = std::env::current_exe().expect("Failed to get current executable");
            let base = executable.parent().expect("Failed to get parent directory");

            cfg_if! {
                if #[cfg(platform_windows)] {
                    base.clean()
                } else if #[cfg(any(platform_linux, platform_bsd))] {
                    base.join("..").join("lib").join("SrednjeveskiArhivi").clean()
                } else if #[cfg(platform_macos)] {
                    base.join("..").join("Resources").clean()
                } else {
                    compile_error!("Unknown operating system")
                }
            }
        };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_clean_windows() {
        assert_eq!(
            Path::new("C:\\Users\\User\\Something").clean(),
            PathBuf::from("C:\\Users\\User\\Something")
        );
        assert_eq!(
            Path::new("C:\\Users\\User\\.\\Something").clean(),
            PathBuf::from("C:\\Users\\User\\Something")
        );
        assert_eq!(
            Path::new("C:\\Users\\User\\..\\Another\\Something").clean(),
            PathBuf::from("C:\\Users\\Another\\Something")
        );
    }

    #[test]
    fn test_path_clean_unix() {
        assert_eq!(
            Path::new("/home/user/something").clean(),
            PathBuf::from("/home/user/something")
        );
        assert_eq!(
            Path::new("/home/user/./something").clean(),
            PathBuf::from("/home/user/something")
        );
        assert_eq!(
            Path::new("/home/user/../another/something").clean(),
            PathBuf::from("/home/another/something")
        );
    }
}
