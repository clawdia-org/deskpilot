use serde::{Deserialize, Serialize};

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
    Click,
    DoubleClick,
    RightClick,
    TripleClick,
    SetValue(String),
    SetFocus,
    Expand,
    Collapse,
    Select(String),
    Toggle,
    Check,
    Uncheck,
    Scroll(Direction, u32),
    ScrollTo,
    PressKey(KeyCombo),
    KeyDown(KeyCombo),
    KeyUp(KeyCombo),
    TypeText(String),
    Clear,
    Hover,
    Drag(DragParams),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DragParams {
    pub from: Point,
    pub to: Point,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseEventKind {
    Move,
    Down,
    Up,
    Click { count: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MouseEvent {
    pub kind: MouseEventKind,
    pub point: Point,
    pub button: MouseButton,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WindowOp {
    Resize { width: f64, height: f64 },
    Move { x: f64, y: f64 },
    Minimize,
    Maximize,
    Restore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyCombo {
    pub key: String,
    pub modifiers: Vec<Modifier>,
}

/// Keyboard modifier keys.
///
/// # Platform Mapping
///
/// | Variant | macOS | Windows | Linux |
/// |---------|-------|---------|-------|
/// | `Cmd`   | ⌘     | Ctrl    | Ctrl  |
/// | `Ctrl`  | ⌃     | Ctrl    | Ctrl  |
/// | `Alt`   | ⌥     | Alt     | Alt   |
/// | `Shift` | ⇧     | Shift   | Shift |
/// | `Meta`  | ⌘     | Win     | Super |
///
/// `Cmd` is preserved for backward compatibility with macOS-first code.
/// For cross-platform key combos, prefer `Meta` which maps to the platform's
/// "super" key (⌘ on macOS, Win on Windows, Super on Linux).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Modifier {
    /// macOS ⌘ key. Maps to Ctrl on Windows/Linux.
    Cmd,
    /// Control key (⌃ on macOS). Consistent across platforms.
    Ctrl,
    /// Alt/Option key (⌥ on macOS). Consistent across platforms.
    Alt,
    /// Shift key. Consistent across platforms.
    Shift,
    /// Platform "super" key: ⌘ on macOS, Win on Windows, Super on Linux.
    /// Use this for cross-platform shortcuts like Meta+C for copy.
    Meta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionResult {
    pub action: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_state: Option<ElementState>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementState {
    pub role: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub states: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ActionResult {
    pub fn new(action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            ref_id: None,
            post_state: None,
        }
    }

    pub fn with_ref(mut self, ref_id: impl Into<String>) -> Self {
        self.ref_id = Some(ref_id.into());
        self
    }

    pub fn with_state(mut self, state: ElementState) -> Self {
        self.post_state = Some(state);
        self
    }
}
