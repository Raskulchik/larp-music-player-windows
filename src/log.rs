use std::io::Write;
use std::path::PathBuf;

fn log_path() -> PathBuf {
    if let Some(path) = std::env::var_os("LARP_LOG_FILE").filter(|v| !v.is_empty()) {
        return PathBuf::from(path);
    }
    std::env::temp_dir().join("music-player-tui.log")
}

pub fn log(msg: &str) {
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_path())
    {
        let _ = writeln!(f, "{}", msg);
    }
}

#[macro_export]
macro_rules! dlog {
    ($($arg:tt)*) => {
        $crate::log::log(&format!($($arg)*))
    };
}
