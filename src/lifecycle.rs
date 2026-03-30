//! Lifecycle subsystem — PID lockfile + zero-downtime hot reload
//!
//! On start: read old PID, write own PID, SIGTERM old process.
//! Deploy becomes: copy new binary, run it. Old one dies.

use std::fs;
use std::path::PathBuf;
use std::process;

/// f13=pid_path — returns the PID lockfile path
pub fn f13() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ghost-fabric")
        .join("pid")
}

/// f14=acquire — write own PID, SIGTERM old process if running
pub fn f14() {
    let path = f13();
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Read old PID
    let old_pid = fs::read_to_string(&path)
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok());

    // Write own PID immediately
    let own_pid = process::id();
    fs::write(&path, own_pid.to_string()).expect("failed to write PID file");

    // Signal old process to exit
    if let Some(pid) = old_pid
        && pid != own_pid
        && pid > 1
    {
        f15(pid);
    }
}

/// f15=signal_old — SIGTERM old PID, wait 5s, SIGKILL if needed
fn f15(pid: u32) {
    #[cfg(unix)]
    {
        use std::time::{Duration, Instant};

        let pid = pid as i32;

        // Check if process exists
        let alive = unsafe { libc::kill(pid, 0) } == 0;
        if !alive {
            return;
        }

        eprintln!("Sending SIGTERM to old instance (PID {})", pid);
        unsafe { libc::kill(pid, libc::SIGTERM) };

        // Wait up to 5 seconds for graceful shutdown
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            std::thread::sleep(Duration::from_millis(100));
            let still_alive = unsafe { libc::kill(pid, 0) } == 0;
            if !still_alive {
                eprintln!("Old instance exited gracefully");
                return;
            }
            if Instant::now() >= deadline {
                break;
            }
        }

        // Force kill
        eprintln!("Old instance didn't exit, sending SIGKILL");
        unsafe { libc::kill(pid, libc::SIGKILL) };
    }
}

/// f16=release — remove PID lockfile on shutdown
pub fn f16() {
    let path = f13();
    let _ = fs::remove_file(&path);
}
