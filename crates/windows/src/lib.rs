#![cfg(target_os = "windows")]

mod system;

use deskpilot_core::{
    action::{Action, ActionResult, DragParams, MouseEvent, WindowOp},
    adapter::{
        ImageBuffer, NativeHandle, PermissionStatus, PlatformAdapter, ScreenshotTarget,
        SnapshotSurface, TreeOptions, WindowFilter,
    },
    error::AdapterError,
    node::{AccessibilityNode, AppInfo, Rect, SurfaceInfo, WindowInfo},
    notification::{NotificationFilter, NotificationIdentity, NotificationInfo},
    refs::RefEntry,
};

pub struct WindowsAdapter;

impl WindowsAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WindowsAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl PlatformAdapter for WindowsAdapter {
    fn check_permissions(&self) -> PermissionStatus {
        crate::system::permissions::check()
    }

    fn list_windows(&self, filter: &WindowFilter) -> Result<Vec<WindowInfo>, AdapterError> {
        crate::system::app_ops::list_windows(filter)
    }

    fn list_apps(&self) -> Result<Vec<AppInfo>, AdapterError> {
        crate::system::app_ops::list_apps()
    }

    fn focus_window(&self, win: &WindowInfo) -> Result<(), AdapterError> {
        crate::system::app_ops::focus_window(win)
    }

    fn launch_app(&self, id: &str, timeout_ms: u64) -> Result<WindowInfo, AdapterError> {
        crate::system::app_ops::launch_app(id, timeout_ms)
    }

    fn close_app(&self, id: &str, force: bool) -> Result<(), AdapterError> {
        crate::system::app_ops::close_app(id, force)
    }

    fn screenshot(&self, target: ScreenshotTarget) -> Result<ImageBuffer, AdapterError> {
        match target {
            ScreenshotTarget::Window(pid) => crate::system::screenshot::capture_app(pid),
            ScreenshotTarget::Screen(idx) => crate::system::screenshot::capture_screen(idx),
            ScreenshotTarget::FullScreen => crate::system::screenshot::capture_screen(0),
        }
    }

    fn press_key_for_app(
        &self,
        app_name: &str,
        combo: &deskpilot_core::action::KeyCombo,
    ) -> Result<ActionResult, AdapterError> {
        crate::system::key_dispatch::press_for_app(app_name, combo)
    }

    fn wait_for_menu(&self, pid: i32, open: bool, timeout_ms: u64) -> Result<(), AdapterError> {
        crate::system::wait::wait_for_menu(pid, open, timeout_ms)
    }

    fn window_op(&self, win: &WindowInfo, op: WindowOp) -> Result<(), AdapterError> {
        crate::system::window_ops::execute(win, op)
    }

    fn mouse_event(&self, event: MouseEvent) -> Result<(), AdapterError> {
        crate::system::input::synthesize_mouse(event)
    }

    fn drag(&self, params: DragParams) -> Result<(), AdapterError> {
        crate::system::input::synthesize_drag(params)
    }

    fn get_clipboard(&self) -> Result<String, AdapterError> {
        crate::system::input::get_clipboard()
    }

    fn set_clipboard(&self, text: &str) -> Result<(), AdapterError> {
        crate::system::input::set_clipboard(text)
    }

    fn clear_clipboard(&self) -> Result<(), AdapterError> {
        crate::system::input::clear_clipboard()
    }

    fn focused_window(&self) -> Result<Option<WindowInfo>, AdapterError> {
        // TODO: Implement Windows focused window detection
        // Use GetForegroundWindow API to get the focused window
        Err(AdapterError::not_supported("focused_window on Windows"))
    }

    fn list_notifications(
        &self,
        filter: &NotificationFilter,
    ) -> Result<Vec<NotificationInfo>, AdapterError> {
        crate::system::notifications::list_notifications(filter)
    }

    fn dismiss_notification(
        &self,
        index: usize,
        app_filter: Option<&str>,
    ) -> Result<NotificationInfo, AdapterError> {
        crate::system::notifications::dismiss_notification(index, app_filter)
    }

    fn dismiss_all_notifications(
        &self,
        app_filter: Option<&str>,
    ) -> Result<(Vec<NotificationInfo>, Vec<String>), AdapterError> {
        crate::system::notifications::dismiss_all_notifications(app_filter)
    }

    fn notification_action(
        &self,
        index: usize,
        identity: Option<&NotificationIdentity>,
        action_name: &str,
    ) -> Result<ActionResult, AdapterError> {
        crate::system::notifications::notification_action(index, identity, action_name)
    }

    // The following methods require UI Automation tree support (Phase 2)

    fn get_tree(
        &self,
        _win: &WindowInfo,
        _opts: &TreeOptions,
    ) -> Result<AccessibilityNode, AdapterError> {
        // TODO: Implement UIA tree traversal
        Err(AdapterError::not_supported("get_tree on Windows"))
    }

    fn execute_action(
        &self,
        _handle: &NativeHandle,
        _action: Action,
    ) -> Result<ActionResult, AdapterError> {
        // TODO: Implement UIA action execution
        Err(AdapterError::not_supported("execute_action on Windows"))
    }

    fn resolve_element(&self, _entry: &RefEntry) -> Result<NativeHandle, AdapterError> {
        // TODO: Implement UIA element resolution
        Err(AdapterError::not_supported("resolve_element on Windows"))
    }

    fn get_live_value(&self, _handle: &NativeHandle) -> Result<Option<String>, AdapterError> {
        // TODO: Implement UIA value retrieval
        Err(AdapterError::not_supported("get_live_value on Windows"))
    }

    fn get_element_bounds(&self, _handle: &NativeHandle) -> Result<Option<Rect>, AdapterError> {
        // TODO: Implement UIA bounds retrieval
        Err(AdapterError::not_supported("get_element_bounds on Windows"))
    }

    fn list_surfaces(&self, _pid: i32) -> Result<Vec<SurfaceInfo>, AdapterError> {
        // TODO: Implement UIA surface enumeration
        Err(AdapterError::not_supported("list_surfaces on Windows"))
    }

    fn get_subtree(
        &self,
        _handle: &NativeHandle,
        _opts: &TreeOptions,
    ) -> Result<AccessibilityNode, AdapterError> {
        // TODO: Implement UIA subtree traversal
        Err(AdapterError::not_supported("get_subtree on Windows"))
    }
}
