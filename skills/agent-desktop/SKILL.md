---
name: deskpilot
version: 0.1.8
tags: desktop-automation, accessibility, ai-agent, gui-automation, cli
requirements:
  - deskpilot
description: >
  Desktop automation via native OS accessibility trees using the deskpilot CLI.
  Use when an AI agent needs to observe, interact with, or automate desktop applications
  (click buttons, fill forms, navigate menus, read UI state, toggle checkboxes, scroll,
  drag, type text, take screenshots, manage windows, use clipboard, manage notifications).
  Covers 53 commands across observation, interaction, keyboard/mouse, app lifecycle,
  notifications (macOS), clipboard, and wait.
  Triggers on: "click button", "fill form", "open app", "read UI", "automate desktop",
  "accessibility tree", "snapshot app", "type into field", "navigate menu", "toggle checkbox",
  "take screenshot", "desktop automation", "deskpilot", or any desktop GUI interaction task.
  Supports macOS (Phase 1), with Windows and Linux planned.
---

# deskpilot

CLI tool enabling AI agents to observe and control desktop applications via native OS accessibility trees.

**Core principle:** deskpilot is NOT an AI agent. It is a tool that AI agents invoke. It outputs structured JSON with ref-based element identifiers. The observation-action loop lives in the calling agent.

## Installation

```bash
npm install -g deskpilot
# or
bun install -g --trust deskpilot
```

Requires macOS 12+ with Accessibility permission granted to your terminal.

## Reference Files

Detailed documentation is split into focused reference files. Read them as needed:

| Reference | Contents |
|-----------|----------|
| `references/commands-observation.md` | snapshot, find, get, is, screenshot, list-surfaces — all flags, output examples |
| `references/commands-interaction.md` | click, type, set-value, select, toggle, scroll, drag, keyboard, mouse — choosing the right command |
| `references/commands-system.md` | launch, close, windows, clipboard, wait, batch, status, permissions, version |
| `references/workflows.md` | 12 common patterns: forms, menus, dialogs, scroll-find, drag-drop, async wait, anti-patterns |
| `references/macos.md` | macOS permissions/TCC, AX API internals, smart activation chain, surfaces, Notification Center, troubleshooting |

## The Observe-Act Loop (Progressive Skeleton Traversal)

Use **progressive skeleton traversal** as the default approach. It reduces token consumption 78-96% for dense apps by exploring the UI in two phases: a shallow skeleton overview, then targeted drill-downs into regions of interest.

```
1. SKELETON → deskpilot snapshot --skeleton --app "App" -i --compact
   Parse the overview. Identify the region containing your target.
   Regions show children_count (e.g., "Sidebar" with children_count: 42).
   Named containers at truncation boundary have refs for drill-down.

2. DRILL    → deskpilot snapshot --root @e3 -i --compact
   Expand the target region. Now you see its interactive elements.

3. ACT      → deskpilot click @e12  (or type, select, toggle...)

4. VERIFY   → deskpilot snapshot --root @e3 -i --compact
   Re-drill the same region to confirm the state change.
   Scoped invalidation: only @e3's subtree refs are replaced.

5. REPEAT   → Continue drilling other regions or acting as needed.
```

**When to skip skeleton and use full snapshot instead:**
- Simple apps with few elements (Finder, Calculator, TextEdit)
- You already know the exact element name — use `find` instead
- Surface snapshots (menus, sheets, alerts) — these are already focused

**When skeleton shines:**
- Dense Electron apps (Slack, VS Code, Discord, Notion)
- Any app where full snapshot exceeds ~50 refs
- Multi-region workflows (sidebar + main content + toolbar)

## Ref System

- Refs assigned depth-first: `@e1`, `@e2`, `@e3`...
- Only interactive elements get refs: button, textfield, checkbox, link, menuitem, tab, slider, combobox, treeitem, cell
- In skeleton mode, named/described containers at truncation boundary also get refs (drill-down targets with empty `available_actions`)
- Static text, groups, containers remain in tree for context but have no ref
- Refs are deterministic within a snapshot but NOT stable across snapshots if UI changed
- After any action that changes UI, re-drill the affected region or re-snapshot
- **Scoped invalidation:** re-drilling `--root @e3` only replaces refs from @e3's previous drill — refs from other regions and the skeleton itself are preserved

## JSON Output Contract

Every command returns a JSON envelope on stdout:

**Success:** `{ "version": "1.0", "ok": true, "command": "snapshot", "data": { ... } }`
**Error:** `{ "version": "1.0", "ok": false, "command": "click", "error": { "code": "STALE_REF", "message": "...", "suggestion": "..." } }`

Exit codes: `0` success, `1` structured error, `2` argument error.

### Error Codes

| Code | Meaning | Recovery |
|------|---------|----------|
| `PERM_DENIED` | Accessibility permission not granted | Grant in System Settings > Privacy > Accessibility |
| `ELEMENT_NOT_FOUND` | Ref not in current refmap | Re-run snapshot, use fresh ref |
| `APP_NOT_FOUND` | App not running | Launch it first |
| `ACTION_FAILED` | AX action rejected | Try alternative approach or coordinate-based click |
| `ACTION_NOT_SUPPORTED` | Element can't do this | Use different command |
| `STALE_REF` | Ref from old snapshot | Re-run snapshot |
| `WINDOW_NOT_FOUND` | No matching window | Check app name, use list-windows |
| `TIMEOUT` | Wait condition not met | Increase --timeout |
| `INVALID_ARGS` | Bad arguments | Check command syntax |

## Command Quick Reference (53 commands)

### Observation
```
deskpilot snapshot --skeleton --app "App" -i --compact  # Skeleton overview (preferred)
deskpilot snapshot --root @e3 -i --compact              # Drill into region
deskpilot snapshot --app "App" -i                       # Full tree (simple apps)
deskpilot snapshot --app "App" --surface menu -i        # Surface snapshot
deskpilot screenshot --app "App" out.png                # PNG screenshot
deskpilot find --app "App" --role button                # Search elements
deskpilot get @e1 --property text                       # Read element property
deskpilot is @e1 --property enabled                     # Check element state
deskpilot list-surfaces --app "App"                     # Available surfaces
```

### Interaction
```
deskpilot click @e5                         # Click element
deskpilot double-click @e3                  # Double-click
deskpilot triple-click @e2                  # Triple-click (select line)
deskpilot right-click @e5                   # Right-click (context menu)
deskpilot type @e2 "hello"                  # Type text into element
deskpilot set-value @e2 "new value"         # Set value directly
deskpilot clear @e2                         # Clear element value
deskpilot focus @e2                         # Set keyboard focus
deskpilot select @e4 "Option B"             # Select dropdown option
deskpilot toggle @e6                        # Toggle checkbox/switch
deskpilot check @e6                         # Idempotent check
deskpilot uncheck @e6                       # Idempotent uncheck
deskpilot expand @e7                        # Expand disclosure
deskpilot collapse @e7                      # Collapse disclosure
deskpilot scroll @e1 --direction down       # Scroll element
deskpilot scroll-to @e8                     # Scroll into view
```

### Keyboard & Mouse
```
deskpilot press cmd+c                       # Key combo
deskpilot press return --app "App"          # Targeted key press
deskpilot key-down shift                    # Hold key
deskpilot key-up shift                      # Release key
deskpilot hover @e5                         # Cursor to element
deskpilot hover --xy 500,300                # Cursor to coordinates
deskpilot drag --from @e1 --to @e5          # Drag between elements
deskpilot mouse-click --xy 500,300          # Click at coordinates
deskpilot mouse-move --xy 100,200           # Move cursor
deskpilot mouse-down --xy 100,200           # Press mouse button
deskpilot mouse-up --xy 300,400             # Release mouse button
```

### App & Window
```
deskpilot launch "System Settings"          # Launch and wait
deskpilot close-app "TextEdit"              # Quit gracefully
deskpilot close-app "TextEdit" --force      # Force kill
deskpilot list-windows --app "Finder"       # List windows
deskpilot list-apps                         # List running GUI apps
deskpilot focus-window --app "Finder"       # Bring to front
deskpilot resize-window --app "App" --width 800 --height 600
deskpilot move-window --app "App" --x 0 --y 0
deskpilot minimize --app "App"
deskpilot maximize --app "App"
deskpilot restore --app "App"
```

### Notifications
```
deskpilot list-notifications                # List all notifications
deskpilot list-notifications --app "Slack"  # Filter by app
deskpilot list-notifications --text "deploy" --limit 5  # Filter by text
deskpilot dismiss-notification 1            # Dismiss by index
deskpilot dismiss-all-notifications         # Dismiss all
deskpilot dismiss-all-notifications --app "Slack"  # Dismiss all from app
deskpilot notification-action 1 "Reply" --expected-app Slack   # Click action (with NC reorder guard)
```

### Clipboard
```
deskpilot clipboard-get                     # Read clipboard
deskpilot clipboard-set "text"              # Write to clipboard
deskpilot clipboard-clear                   # Clear clipboard
```

### Wait
```
deskpilot wait 1000                         # Pause 1 second
deskpilot wait --element @e5 --timeout 5000 # Wait for element
deskpilot wait --window "Title"             # Wait for window
deskpilot wait --text "Done" --app "App"    # Wait for text
deskpilot wait --menu --app "App"           # Wait for context menu
deskpilot wait --menu-closed --app "App"    # Wait for menu dismissal
deskpilot wait --notification --app "App"   # Wait for new notification
```

### System
```
deskpilot status                            # Health check
deskpilot permissions                       # Check permission
deskpilot permissions --request             # Trigger permission dialog
deskpilot version --json                    # Version info
deskpilot batch '[...]' --stop-on-error     # Batch commands
```

## Key Principles for Agents

1. **Skeleton first, drill second.** Start with `--skeleton -i --compact` for dense apps. Drill into regions with `--root @ref`. Full snapshot only for simple apps.
2. **Use `-i --compact` flags.** Filters to interactive elements and collapses empty wrappers, minimizing tokens.
3. **Refs are ephemeral.** Re-drill the affected region after any UI-changing action. Scoped invalidation keeps other refs intact.
4. **Prefer refs over coordinates.** `click @e5` > `mouse-click --xy 500,300`.
5. **Use `wait` for async UI.** After launch/dialog triggers, wait for expected state.
6. **Check permissions first.** Run `permissions` on first use.
7. **Handle errors.** Parse `error.code` and follow `error.suggestion`.
8. **Use `find` for targeted searches.** Faster than any snapshot when you know role/name.
9. **Use surfaces for overlays.** `snapshot --surface menu` for menus, `--surface sheet` for dialogs. Never `--skeleton` for surfaces — they're already focused.
10. **Batch for performance.** Multiple commands in one invocation.
