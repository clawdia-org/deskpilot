use deskpilot_core::{
    adapter::WindowFilter,
    error::AdapterError,
    node::{AppInfo, WindowInfo},
};

/// List all visible windows on the system.
pub fn list_windows(_filter: &WindowFilter) -> Result<Vec<WindowInfo>, AdapterError> {
    // TODO: Implement Windows window enumeration
    // Use EnumWindows API to list windows
    // Filter by app name if provided
    Err(AdapterError::not_supported("list_windows on Windows"))
}

/// List all running applications.
pub fn list_apps() -> Result<Vec<AppInfo>, AdapterError> {
    // TODO: Implement Windows app enumeration
    // Use WMI or Process API to list running apps
    Err(AdapterError::not_supported("list_apps on Windows"))
}

/// Focus a specific window.
pub fn focus_window(_win: &WindowInfo) -> Result<(), AdapterError> {
    // TODO: Implement Windows window focus
    // Use SetForegroundWindow API
    Err(AdapterError::not_supported("focus_window on Windows"))
}

/// Launch an application by name or path.
pub fn launch_app(_id: &str, _timeout_ms: u64) -> Result<WindowInfo, AdapterError> {
    // TODO: Implement Windows app launch
    // Use CreateProcess or ShellExecute
    // Wait for window to appear with timeout
    Err(AdapterError::not_supported("launch_app on Windows"))
}

/// Close an application gracefully or forcefully.
pub fn close_app(_id: &str, _force: bool) -> Result<(), AdapterError> {
    // TODO: Implement Windows app close
    // Use PostMessage(WM_CLOSE) for graceful close
    // Use TerminateProcess for force close
    Err(AdapterError::not_supported("close_app on Windows"))
}
