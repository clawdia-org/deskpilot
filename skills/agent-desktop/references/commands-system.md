# System Commands

App lifecycle, window management, notifications, clipboard, wait, and system health commands.

## App Lifecycle

### launch
```bash
deskpilot launch "System Settings"
deskpilot launch "com.apple.Safari" --timeout 10000
```
Launches an application by name or bundle ID and waits until its window is visible.

| Flag | Default | Description |
|------|---------|-------------|
| `--timeout` | 30000 | Max wait time in ms for window to appear |

### close-app
```bash
deskpilot close-app "TextEdit"
deskpilot close-app "TextEdit" --force
```
Quits an application gracefully. Use `--force` to kill the process.

### list-apps
```bash
deskpilot list-apps
```
Lists all running GUI applications. Returns array of `{ name, pid, bundle_id }`.

## Window Management

### list-windows
```bash
deskpilot list-windows
deskpilot list-windows --app "Finder"
```
Lists all visible windows, optionally filtered by app. Returns array of `{ id, title, app_name, pid, bounds }`.

### focus-window
```bash
deskpilot focus-window --app "Finder"
deskpilot focus-window --title "Documents"
deskpilot focus-window --window-id "w-4521"
```
Brings a window to the front. At least one identifier required.

### resize-window
```bash
deskpilot resize-window --app "TextEdit" --width 800 --height 600
```

### move-window
```bash
deskpilot move-window --app "TextEdit" --x 0 --y 0
```

### minimize
```bash
deskpilot minimize --app "TextEdit"
```

### maximize
```bash
deskpilot maximize --app "TextEdit"
```
Zooms the window to fill the screen.

### restore
```bash
deskpilot restore --app "TextEdit"
```
Restores a minimized or maximized window to its previous size.

## Notifications

### list-notifications
```bash
deskpilot list-notifications
deskpilot list-notifications --app "Slack"
deskpilot list-notifications --text "deploy" --limit 5
```
Lists notifications in the Notification Center. Returns array of `{ index, app_name, title, body, actions }`.

| Flag | Default | Description |
|------|---------|-------------|
| `--app` | | Filter by source app name |
| `--text` | | Filter by text content (matches title and body) |
| `--limit` | | Max number of notifications to return |

### dismiss-notification
```bash
deskpilot dismiss-notification 1
deskpilot dismiss-notification 3 --app "Slack"
```
Dismisses a single notification by its 1-based index. Returns the dismissed notification info.

| Flag | Default | Description |
|------|---------|-------------|
| (positional) | | 1-based notification index (required) |
| `--app` | | Filter by app before indexing |

### dismiss-all-notifications
```bash
deskpilot dismiss-all-notifications
deskpilot dismiss-all-notifications --app "Slack"
```
Dismisses all notifications, optionally filtered by app. Reports per-notification failures.

Returns `{ "dismissed_count": N, "failures": [...], "failed_count": N }`.

### notification-action
```bash
deskpilot notification-action 1 "Reply"
deskpilot notification-action 2 "Mark as Read" --expected-app Slack --expected-title "#general"
```
Clicks a named action button on a notification by its 1-based index.

`--expected-app` and `--expected-title` pin the call to the notification
you observed in `list-notifications`. Notification Center reorders
entries between listings, so without a fingerprint an arriving or
dismissed notification can shift the target at `INDEX` and cause the
action to press the wrong row. When either flag is set and the row at
`INDEX` no longer matches, the call fails with `NOTIFICATION_NOT_FOUND`
instead of pressing. Both flags omitted preserves the legacy
index-only behavior for callers that reconcile themselves.

| Flag | Default | Description |
|------|---------|-------------|
| `INDEX` (positional) | | 1-based notification index (required) |
| `ACTION` (positional) | | Action button name to click (required) |
| `--expected-app` | | Fingerprint app name (from `list-notifications`) |
| `--expected-title` | | Fingerprint title (from `list-notifications`) |

### wait --notification
```bash
deskpilot wait --notification --app "App" --timeout 10000
deskpilot wait --notification --text "build passed" --timeout 15000
```
Blocks until a new notification appears (detects index-diff from previous state). Supports `--app` and `--text` filters.

## Clipboard

### clipboard-get
```bash
deskpilot clipboard-get
```
Returns `{ "data": { "text": "clipboard contents" } }`.

### clipboard-set
```bash
deskpilot clipboard-set "Hello, world!"
```

### clipboard-clear
```bash
deskpilot clipboard-clear
```

## Wait

### wait (time)
```bash
deskpilot wait 1000
```
Pauses for N milliseconds. Use between actions that need time to settle.

### wait (element)
```bash
deskpilot wait --element @e5 --timeout 5000 --app "App"
```
Blocks until the element ref appears in the accessibility tree. Useful after triggering UI changes.

### wait (window)
```bash
deskpilot wait --window "Save As" --timeout 10000
```
Blocks until a window with the given title appears.

### wait (text)
```bash
deskpilot wait --text "Loading complete" --app "Safari" --timeout 5000
```
Blocks until the specified text appears anywhere in the app's accessibility tree.

### wait (menu)
```bash
deskpilot wait --menu --app "Finder" --timeout 3000
```
Blocks until a context menu is detected as open.

### wait (menu-closed)
```bash
deskpilot wait --menu-closed --app "Finder" --timeout 3000
```
Blocks until the context menu is dismissed.

| Flag | Default | Description |
|------|---------|-------------|
| (positional) | | Milliseconds to pause |
| `--element` | | Ref to wait for |
| `--window` | | Window title to wait for |
| `--text` | | Text to wait for |
| `--menu` | false | Wait for context menu to open |
| `--menu-closed` | false | Wait for context menu to close |
| `--timeout` | 30000 | Timeout in ms (for element/window/text/menu waits) |
| `--app` | | Scope the wait to a specific application |

## Batch

### batch
```bash
deskpilot batch '[{"command":"click","args":{"ref_id":"@e1"}},{"command":"wait","args":{"ms":500}},{"command":"click","args":{"ref_id":"@e2"}}]'
deskpilot batch '[...]' --stop-on-error
```
Execute multiple commands in sequence from a JSON array. Each entry has `command` (string) and `args` (object).

| Flag | Default | Description |
|------|---------|-------------|
| `--stop-on-error` | false | Halt on first failed command |

**Batch format:**
```json
[
  { "command": "click", "args": { "ref_id": "@e1" } },
  { "command": "wait", "args": { "ms": 500 } },
  { "command": "type", "args": { "ref_id": "@e2", "text": "hello" } }
]
```

**Progressive snapshot in batch** — use `skeleton` and `root` fields inside `snapshot` args:
```json
[
  { "command": "snapshot", "args": { "app": "Slack", "skeleton": true, "interactive_only": true } },
  { "command": "snapshot", "args": { "app": "Slack", "root": "@e3", "interactive_only": true } }
]
```

`skeleton: true` clamps depth to 3 and tags truncated containers with `children_count`. `root: "@eN"` starts traversal from that ref instead of the window root; it cannot be combined with `surface`.

## System Health

### status
```bash
deskpilot status
```
Returns adapter health, platform info, and permission state.

### permissions
```bash
deskpilot permissions
deskpilot permissions --request
```
Checks accessibility permission status. Use `--request` to trigger the macOS system dialog.

### version
```bash
deskpilot version
deskpilot version --json
```
Returns version string. Use `--json` for `{ "version": "0.1.3", "platform": "macos", "arch": "aarch64" }`.
