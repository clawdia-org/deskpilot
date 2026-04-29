use deskpilot_core::{
    action::ActionResult,
    error::AdapterError,
    notification::{NotificationFilter, NotificationIdentity, NotificationInfo},
};

/// List notifications from the Windows Action Center.
pub fn list_notifications(
    _filter: &NotificationFilter,
) -> Result<Vec<NotificationInfo>, AdapterError> {
    // TODO: Implement Windows notification listing
    // Use Windows.UI.Notifications API or UIA to access Action Center
    Err(AdapterError::not_supported("list_notifications on Windows"))
}

/// Dismiss a notification by index.
pub fn dismiss_notification(
    _index: usize,
    _app_filter: Option<&str>,
) -> Result<NotificationInfo, AdapterError> {
    // TODO: Implement Windows notification dismiss
    Err(AdapterError::not_supported(
        "dismiss_notification on Windows",
    ))
}

/// Dismiss all notifications, optionally filtered by app.
pub fn dismiss_all_notifications(
    _app_filter: Option<&str>,
) -> Result<(Vec<NotificationInfo>, Vec<String>), AdapterError> {
    // TODO: Implement Windows dismiss all notifications
    Err(AdapterError::not_supported(
        "dismiss_all_notifications on Windows",
    ))
}

/// Trigger an action on a notification.
pub fn notification_action(
    _index: usize,
    _identity: Option<&NotificationIdentity>,
    _action_name: &str,
) -> Result<ActionResult, AdapterError> {
    // TODO: Implement Windows notification action
    Err(AdapterError::not_supported(
        "notification_action on Windows",
    ))
}
