use deskpilot_core::{action::{DragParams, MouseEvent}, error::AdapterError};

/// Synthesize a mouse event (move, click, scroll).
pub fn synthesize_mouse(_event: MouseEvent) -> Result<(), AdapterError> {
    // TODO: Implement Windows mouse synthesis
    // Use SendInput API with INPUT_MOUSE structure
    Err(AdapterError::not_supported("mouse_event on Windows"))
}

/// Synthesize a drag operation.
pub fn synthesize_drag(_params: DragParams) -> Result<(), AdapterError> {
    // TODO: Implement Windows drag synthesis
    // Use SendInput API with multiple mouse events
    Err(AdapterError::not_supported("drag on Windows"))
}

/// Get clipboard text.
pub fn get_clipboard() -> Result<String, AdapterError> {
    // TODO: Implement Windows clipboard get
    // Use GetClipboardData with CF_UNICODETEXT format
    Err(AdapterError::not_supported("get_clipboard on Windows"))
}

/// Set clipboard text.
pub fn set_clipboard(_text: &str) -> Result<(), AdapterError> {
    // TODO: Implement Windows clipboard set
    // Use SetClipboardData with CF_UNICODETEXT format
    Err(AdapterError::not_supported("set_clipboard on Windows"))
}

/// Clear the clipboard.
pub fn clear_clipboard() -> Result<(), AdapterError> {
    // TODO: Implement Windows clipboard clear
    // Use EmptyClipboard
    Err(AdapterError::not_supported("clear_clipboard on Windows"))
}
