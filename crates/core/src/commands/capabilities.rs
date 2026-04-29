use crate::error::AppError;
use serde_json::{json, Value};

pub struct CapabilitiesArgs;

pub fn execute(_args: CapabilitiesArgs) -> Result<Value, AppError> {
    let os = std::env::consts::OS;

    let capabilities = match os {
        "macos" => json!({
            "platform": "macos",
            "commands": [
                "snapshot", "find", "screenshot", "get", "is",
                "click", "double_click", "triple_click", "right_click",
                "type", "set_value", "clear", "focus", "select", "toggle",
                "press", "key_down", "key_up",
                "mouse_move", "mouse_click", "mouse_down", "mouse_up",
                "drag", "scroll", "scroll_to", "hover",
                "launch", "close_app", "list_apps",
                "list_windows", "list_surfaces", "focus_window",
                "move_window", "resize_window", "minimize", "maximize", "restore",
                "screenshot", "permissions", "wait",
                "list_notifications", "dismiss_notification", "dismiss_all_notifications", "notification_action",
                "clipboard_get", "clipboard_set", "clipboard_clear",
                "expand", "collapse", "check", "uncheck",
                "batch", "status", "version", "capabilities"
            ],
            "features": {
                "accessibility_tree": true,
                "element_interaction": true,
                "window_management": true,
                "notifications": true,
                "clipboard": true,
                "keyboard_input": true,
                "mouse_input": true,
                "screenshots": true,
                "batch_commands": true
            },
            "limitations": []
        }),
        "windows" => json!({
            "platform": "windows",
            "commands": [
                "snapshot", "find", "screenshot", "get", "is",
                "version", "capabilities", "permissions", "status"
            ],
            "features": {
                "accessibility_tree": true,
                "element_interaction": false,
                "window_management": false,
                "notifications": false,
                "clipboard": false,
                "keyboard_input": false,
                "mouse_input": false,
                "screenshots": false,
                "batch_commands": false
            },
            "limitations": [
                "Windows adapter is in Phase 2 development",
                "Only observation commands (snapshot, find, screenshot) are implemented",
                "Interaction commands (click, type, etc.) not yet available"
            ]
        }),
        "linux" => json!({
            "platform": "linux",
            "commands": [
                "snapshot", "find", "screenshot", "get", "is",
                "version", "capabilities", "permissions", "status"
            ],
            "features": {
                "accessibility_tree": true,
                "element_interaction": false,
                "window_management": false,
                "notifications": false,
                "clipboard": false,
                "keyboard_input": false,
                "mouse_input": false,
                "screenshots": false,
                "batch_commands": false
            },
            "limitations": [
                "Linux adapter is in Phase 2 development",
                "Only observation commands (snapshot, find, screenshot) are implemented",
                "Interaction commands (click, type, etc.) not yet available"
            ]
        }),
        _ => json!({
            "platform": os,
            "commands": ["version", "capabilities", "status"],
            "features": {},
            "limitations": ["Unknown platform"]
        }),
    };

    Ok(json!({
        "version": env!("CARGO_PKG_VERSION"),
        "capabilities": capabilities
    }))
}
