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
}

#[cfg(not(target_os = "windows"))]
mod tests {
    #[test]
    fn test_noop_on_non_windows() {
        // Placeholder test for non-Windows platforms
        assert!(true);
    }
}
