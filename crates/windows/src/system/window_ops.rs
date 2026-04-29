use deskpilot_core::{action::WindowOp, error::AdapterError, node::WindowInfo};

/// Execute a window operation (minimize, maximize, close, etc).
pub fn execute(_win: &WindowInfo, _op: WindowOp) -> Result<(), AdapterError> {
    // TODO: Implement Windows window operations
    // Use ShowWindow API with appropriate flags:
    // - SW_MINIMIZE (6)
    // - SW_MAXIMIZE (3)
    // - SW_RESTORE (9)
    // - WM_CLOSE message for close
    Err(AdapterError::not_supported("window_op on Windows"))
}
