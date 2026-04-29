use crate::{
    action::{Action, KeyCombo, Modifier},
    adapter::PlatformAdapter,
    error::AppError,
};
use serde_json::Value;

const BLOCKED_COMBOS: &[&str] = &[
    "cmd+q",
    "cmd+shift+q",
    "cmd+alt+esc",
    "ctrl+cmd+q",
    "cmd+shift+delete",
];

pub struct PressArgs {
    pub combo: String,
    pub app: Option<String>,
}

pub fn execute(args: PressArgs, adapter: &dyn PlatformAdapter) -> Result<Value, AppError> {
    let normalized = args.combo.to_lowercase().replace(' ', "");
    if BLOCKED_COMBOS.contains(&normalized.as_str()) {
        return Err(AppError::invalid_input(format!(
            "Key combo '{}' is blocked for safety",
            args.combo
        )));
    }

    let combo = parse_combo(&normalized)?;

    if let Some(app_name) = &args.app {
        let result = adapter.press_key_for_app(app_name, &combo)?;
        return Ok(serde_json::to_value(result)?);
    }

    let handle = crate::adapter::NativeHandle::null();
    let result = adapter.execute_action(&handle, Action::PressKey(combo))?;
    Ok(serde_json::to_value(result)?)
}

/// Parse a key combo string like "cmd+shift+s" or "meta+c".
///
/// # Supported Modifiers
///
/// - `cmd`, `command` → `Modifier::Cmd` (macOS ⌘, maps to Ctrl on Windows/Linux)
/// - `ctrl`, `control` → `Modifier::Ctrl` (consistent across platforms)
/// - `alt`, `option` → `Modifier::Alt` (consistent across platforms)
/// - `shift` → `Modifier::Shift` (consistent across platforms)
/// - `meta`, `super`, `win` → `Modifier::Meta` (platform super key)
///
/// # Examples
///
/// ```ignore
/// parse_combo("cmd+s")?;      // macOS save
/// parse_combo("ctrl+s")?;     // Windows/Linux save
/// parse_combo("meta+c")?;     // Cross-platform copy
/// ```
pub fn parse_combo(s: &str) -> Result<KeyCombo, AppError> {
    let parts: Vec<&str> = s.split('+').collect();
    let key = parts
        .last()
        .copied()
        .filter(|k| !k.is_empty())
        .ok_or_else(|| AppError::invalid_input("Empty key combo"))?
        .to_string();
    let mut modifiers = Vec::new();

    for &part in &parts[..parts.len() - 1] {
        let modifier = match part {
            "cmd" | "command" => Modifier::Cmd,
            "ctrl" | "control" => Modifier::Ctrl,
            "alt" | "option" => Modifier::Alt,
            "shift" => Modifier::Shift,
            "meta" | "super" | "win" => Modifier::Meta,
            other => {
                return Err(AppError::invalid_input(format!(
                    "Unknown modifier: '{other}'. Use: cmd, ctrl, alt, shift, meta"
                )))
            }
        };
        modifiers.push(modifier);
    }

    Ok(KeyCombo { key, modifiers })
}
