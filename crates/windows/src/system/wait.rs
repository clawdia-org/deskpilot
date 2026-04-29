use deskpilot_core::error::AdapterError;

/// Wait for a context menu to open or close on Windows.
///
/// On Windows, we can detect menus by:
/// 1. Polling the UIA tree for menu elements
/// 2. Checking for popup windows with specific styles
/// 3. Monitoring window messages
pub fn wait_for_menu(_pid: i32, _open: bool, _timeout_ms: u64) -> Result<(), AdapterError> {
    // TODO: Implement Windows menu detection
    // Use UIA to detect menu elements or poll window styles
    Err(AdapterError::not_supported("wait_for_menu on Windows"))
}
