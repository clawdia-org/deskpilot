use deskpilot_core::adapter::PermissionStatus;

/// Check if the process has UI Automation permissions on Windows.
///
/// On Windows, UI Automation access is typically granted if:
/// 1. Running as administrator
/// 2. Running from a trusted location
/// 3. The application is in the accessibility allowlist
///
/// For now, we return Granted since Windows doesn't have the same
/// permission model as macOS. In production, this should check
/// actual UIA access and suggest enabling if needed.
pub fn check() -> PermissionStatus {
    // TODO: Implement actual UIA permission check
    // For Phase 1, assume granted on Windows
    PermissionStatus::Granted
}
