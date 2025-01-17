use std::panic;

use crate::command::Command;
use crate::env_var;
use crate::util::handle_failed_output;

/// `TARGET`
#[must_use]
pub fn target() -> String {
    env_var("TARGET")
}

/// Check if target is windows-like.
#[must_use]
pub fn is_windows() -> bool {
    target().contains("windows")
}

/// Check if target uses msvc.
#[must_use]
pub fn is_msvc() -> bool {
    target().contains("msvc")
}

/// Check if target uses macOS.
#[must_use]
pub fn is_darwin() -> bool {
    target().contains("darwin")
}

/// Run `uname`. This assumes that `uname` is available on the platform!
#[track_caller]
#[must_use]
pub fn uname() -> String {
    let caller = panic::Location::caller();
    let mut uname = Command::new("uname");
    let output = uname.run();
    if !output.status().success() {
        handle_failed_output(&uname, output, caller.line());
    }
    output.stdout_utf8()
}
