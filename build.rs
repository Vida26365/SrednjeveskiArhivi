use cfg_aliases::cfg_aliases;

fn main() {
    cfg_aliases! {
        platform_windows: { target_os = "windows" },
        platform_linux: { target_os = "linux" },
        platform_macos: { target_os = "macos" },
        platform_bsd: { any(target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd", target_os = "netbsd") },
    }
}
