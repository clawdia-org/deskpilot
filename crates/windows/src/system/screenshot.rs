use deskpilot_core::{adapter::ImageBuffer, adapter::ImageFormat, error::AdapterError};

/// Capture a screenshot of a specific window by PID.
pub fn capture_app(_pid: i32) -> Result<ImageBuffer, AdapterError> {
    // TODO: Implement Windows screenshot for specific window
    // Use Windows GDI or DXGI to capture window by HWND
    Err(AdapterError::not_supported("capture_app on Windows"))
}

/// Capture a screenshot of a specific screen by index.
pub fn capture_screen(_idx: usize) -> Result<ImageBuffer, AdapterError> {
    // TODO: Implement Windows screenshot for screen
    // Use Windows GDI or DXGI to capture display
    Err(AdapterError::not_supported("capture_screen on Windows"))
}
