use deskpilot_core::{
    action::{ActionResult, KeyCombo},
    error::AdapterError,
};

/// Map key names to Windows Virtual Key codes.
///
/// # Virtual Key Mapping
///
/// Common keys:
/// - "a"-"z", "0"-"9" → VK_A-VK_Z, VK_0-VK_9
/// - "return"/"enter" → VK_RETURN (0x0D)
/// - "escape"/"esc" → VK_ESCAPE (0x1B)
/// - "space" → VK_SPACE (0x20)
/// - "tab" → VK_TAB (0x09)
/// - "backspace" → VK_BACK (0x08)
/// - "delete" → VK_DELETE (0x2E)
/// - "insert" → VK_INSERT (0x2D)
/// - "home" → VK_HOME (0x24)
/// - "end" → VK_END (0x23)
/// - "pageup" → VK_PRIOR (0x21)
/// - "pagedown" → VK_NEXT (0x22)
/// - "up"/"down"/"left"/"right" → VK_UP/DOWN/LEFT/RIGHT (0x26-0x29)
/// - "f1"-"f12" → VK_F1-VK_F12 (0x70-0x7B)
///
/// See: https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
pub fn key_name_to_vk(key: &str) -> Option<u8> {
    match key.to_lowercase().as_str() {
        // Letters
        "a" => Some(0x41),
        "b" => Some(0x42),
        "c" => Some(0x43),
        "d" => Some(0x44),
        "e" => Some(0x45),
        "f" => Some(0x46),
        "g" => Some(0x47),
        "h" => Some(0x48),
        "i" => Some(0x49),
        "j" => Some(0x4A),
        "k" => Some(0x4B),
        "l" => Some(0x4C),
        "m" => Some(0x4D),
        "n" => Some(0x4E),
        "o" => Some(0x4F),
        "p" => Some(0x50),
        "q" => Some(0x51),
        "r" => Some(0x52),
        "s" => Some(0x53),
        "t" => Some(0x54),
        "u" => Some(0x55),
        "v" => Some(0x56),
        "w" => Some(0x57),
        "x" => Some(0x58),
        "y" => Some(0x59),
        "z" => Some(0x5A),
        // Numbers
        "0" => Some(0x30),
        "1" => Some(0x31),
        "2" => Some(0x32),
        "3" => Some(0x33),
        "4" => Some(0x34),
        "5" => Some(0x35),
        "6" => Some(0x36),
        "7" => Some(0x37),
        "8" => Some(0x38),
        "9" => Some(0x39),
        // Special keys
        "return" | "enter" => Some(0x0D),
        "escape" | "esc" => Some(0x1B),
        "space" => Some(0x20),
        "tab" => Some(0x09),
        "backspace" | "back" => Some(0x08),
        "delete" | "del" => Some(0x2E),
        "insert" | "ins" => Some(0x2D),
        "home" => Some(0x24),
        "end" => Some(0x23),
        "pageup" | "prior" => Some(0x21),
        "pagedown" | "next" => Some(0x22),
        // Arrow keys
        "up" => Some(0x26),
        "down" => Some(0x28),
        "left" => Some(0x25),
        "right" => Some(0x27),
        // Function keys
        "f1" => Some(0x70),
        "f2" => Some(0x71),
        "f3" => Some(0x72),
        "f4" => Some(0x73),
        "f5" => Some(0x74),
        "f6" => Some(0x75),
        "f7" => Some(0x76),
        "f8" => Some(0x77),
        "f9" => Some(0x78),
        "f10" => Some(0x79),
        "f11" => Some(0x7A),
        "f12" => Some(0x7B),
        // Symbols (US keyboard layout)
        ";" | "colon" => Some(0xBA),
        "=" | "plus" => Some(0xBB),
        "," | "comma" => Some(0xBC),
        "-" | "minus" => Some(0xBD),
        "." | "period" => Some(0xBE),
        "/" | "slash" => Some(0xBF),
        "`" | "backtick" => Some(0xC0),
        "[" | "lbracket" => Some(0xDB),
        "\\" | "backslash" => Some(0xDC),
        "]" | "rbracket" => Some(0xDD),
        "'" | "quote" => Some(0xDE),
        _ => None,
    }
}

/// Press a key combination for a specific application.
pub fn press_for_app(_app_name: &str, _combo: &KeyCombo) -> Result<ActionResult, AdapterError> {
    // TODO: Implement Windows key press for app
    // 1. Find window by app name
    // 2. Focus the window
    // 3. Synthesize keyboard input with modifiers
    Err(AdapterError::not_supported("press_key_for_app on Windows"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_name_to_vk_letters() {
        assert_eq!(key_name_to_vk("a"), Some(0x41));
        assert_eq!(key_name_to_vk("Z"), Some(0x5A));
        assert_eq!(key_name_to_vk("m"), Some(0x4D));
    }

    #[test]
    fn test_key_name_to_vk_numbers() {
        assert_eq!(key_name_to_vk("0"), Some(0x30));
        assert_eq!(key_name_to_vk("9"), Some(0x39));
    }

    #[test]
    fn test_key_name_to_vk_special() {
        assert_eq!(key_name_to_vk("return"), Some(0x0D));
        assert_eq!(key_name_to_vk("enter"), Some(0x0D));
        assert_eq!(key_name_to_vk("escape"), Some(0x1B));
        assert_eq!(key_name_to_vk("esc"), Some(0x1B));
        assert_eq!(key_name_to_vk("space"), Some(0x20));
        assert_eq!(key_name_to_vk("tab"), Some(0x09));
        assert_eq!(key_name_to_vk("backspace"), Some(0x08));
        assert_eq!(key_name_to_vk("delete"), Some(0x2E));
    }

    #[test]
    fn test_key_name_to_vk_arrows() {
        assert_eq!(key_name_to_vk("up"), Some(0x26));
        assert_eq!(key_name_to_vk("down"), Some(0x28));
        assert_eq!(key_name_to_vk("left"), Some(0x25));
        assert_eq!(key_name_to_vk("right"), Some(0x27));
    }

    #[test]
    fn test_key_name_to_vk_function_keys() {
        assert_eq!(key_name_to_vk("f1"), Some(0x70));
        assert_eq!(key_name_to_vk("f12"), Some(0x7B));
    }

    #[test]
    fn test_key_name_to_vk_unknown() {
        assert_eq!(key_name_to_vk("unknown"), None);
        assert_eq!(key_name_to_vk(""), None);
    }

    #[test]
    fn test_key_name_to_vk_case_insensitive() {
        assert_eq!(key_name_to_vk("A"), Some(0x41));
        assert_eq!(key_name_to_vk("RETURN"), Some(0x0D));
        assert_eq!(key_name_to_vk("F1"), Some(0x70));
    }
}
