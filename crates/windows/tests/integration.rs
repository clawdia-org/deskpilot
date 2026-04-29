//! Windows integration tests scaffold.
//!
//! These tests verify Windows-specific functionality using UI Automation.
//! They only run on Windows targets and require a desktop session.

#[cfg(target_os = "windows")]
mod tests {
    use deskpilot_windows::WindowsAdapter;
    use deskpilot_core::adapter::PlatformAdapter;

    fn get_adapter() -> WindowsAdapter {
        WindowsAdapter::new()
    }

    #[test]
    fn test_permissions_check() {
        let adapter = get_adapter();
        let status = adapter.check_permissions();
        // On Windows, permissions should always be granted
        assert!(matches!(status, deskpilot_core::adapter::PermissionStatus::Granted));
    }

    #[test]
    fn test_list_apps() {
        let adapter = get_adapter();
        let apps = adapter.list_apps().expect("list_apps should work");
        // Should have at least some running apps
        assert!(!apps.is_empty(), "Should have at least one running app");
    }

    #[test]
    fn test_list_windows() {
        let adapter = get_adapter();
        use deskpilot_core::adapter::WindowFilter;
        let filter = WindowFilter {
            focused_only: false,
            app: None,
        };
        let windows = adapter.list_windows(&filter).expect("list_windows should work");
        // Should have at least some visible windows
        assert!(!windows.is_empty(), "Should have at least one visible window");
    }

    #[test]
    fn test_screenshot_full_screen() {
        let adapter = get_adapter();
        use deskpilot_core::adapter::ScreenshotTarget;
        let result = adapter.screenshot(ScreenshotTarget::FullScreen);
        assert!(result.is_ok(), "Screenshot should succeed");
        let img = result.unwrap();
        assert!(!img.data.is_empty(), "Screenshot data should not be empty");
        assert!(img.width > 0, "Screenshot width should be positive");
        assert!(img.height > 0, "Screenshot height should be positive");
    }

    #[test]
    fn test_key_name_to_vk() {
        use crate::system::key_dispatch::key_name_to_vk;

        // Test letter keys
        assert_eq!(key_name_to_vk("a"), Some(0x41));
        assert_eq!(key_name_to_vk("Z"), Some(0x5A));

        // Test number keys
        assert_eq!(key_name_to_vk("0"), Some(0x30));
        assert_eq!(key_name_to_vk("9"), Some(0x39));

        // Test special keys
        assert_eq!(key_name_to_vk("return"), Some(0x0D));
        assert_eq!(key_name_to_vk("enter"), Some(0x0D));
        assert_eq!(key_name_to_vk("escape"), Some(0x1B));
        assert_eq!(key_name_to_vk("space"), Some(0x20));

        // Test function keys
        assert_eq!(key_name_to_vk("f1"), Some(0x70));
        assert_eq!(key_name_to_vk("f12"), Some(0x7B));

        // Test unknown key
        assert_eq!(key_name_to_vk("unknown"), None);
    }
}

#[cfg(not(target_os = "windows"))]
mod tests {
    #[test]
    fn test_noop_on_non_windows() {
        // Placeholder test for non-Windows platforms
        assert!(true);
    }
}
