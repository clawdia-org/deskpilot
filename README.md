<h1 align="center">DESKPILOT</h1>

<p align="center">
  <strong>OBSERVE. DECIDE. ACT.</strong>
</p>

<p align="center">
  <a href="https://github.com/clawdia-org/deskpilot/actions/workflows/ci.yml?query=branch%3Amain"><img src="https://img.shields.io/github/actions/workflow/status/clawdia-org/deskpilot/ci.yml?branch=main&style=for-the-badge" alt="CI status"></a>
  <a href="https://github.com/clawdia-org/deskpilot/releases"><img src="https://img.shields.io/github/v/release/clawdia-org/deskpilot?include_prereleases&style=for-the-badge" alt="GitHub release"></a>
  <a href="https://www.npmjs.com/package/deskpilot"><img src="https://img.shields.io/npm/v/deskpilot?label=npm&style=for-the-badge" alt="npm version"></a>
  <a href="https://clawhub.ai/clawdia-org/deskpilot"><img src="https://img.shields.io/badge/ClawHub-skill-f97316?style=for-the-badge" alt="ClawHub skill"></a>
  <a href="https://skills.sh/clawdia-org/deskpilot/deskpilot"><img src="https://img.shields.io/badge/skills.sh-listed-8b5cf6?style=for-the-badge" alt="skills.sh listing"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/License-Apache--2.0-blue.svg?style=for-the-badge" alt="Apache-2.0 License"></a>
</p>

**deskpilot** is a native desktop automation CLI designed for AI agents, built with Rust. It gives structured access to any application through OS accessibility trees — no screenshots, no pixel matching, no browser required.

## Architecture

<p align="center">
  <img src="docs/architecture.png" alt="deskpilot architecture diagram" width="800" />
</p>

<a href="https://star-history.com/#clawdia-org/deskpilot&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=clawdia-org/deskpilot&type=Date&theme=dark">
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=clawdia-org/deskpilot&type=Date">
    <img alt="Star history for clawdia-org/deskpilot" src="https://api.star-history.com/svg?repos=clawdia-org/deskpilot&type=Date">
  </picture>
</a>

## Key Features

- **Native Rust CLI**: Fast, single binary, no runtime dependencies
- **C-ABI cdylib** (`libdeskpilot_ffi`): Load once from Python / Swift / Go / Ruby / Node / C instead of forking the CLI per call
- **53 commands**: Observation, interaction, keyboard, mouse, notifications, clipboard, window management
- **Progressive skeleton traversal**: 78–96% token reduction on dense apps via shallow overview + targeted drill-down
- **Snapshot & refs**: AI-optimized workflow using deterministic element references (`@e1`, `@e2`)
- **AX-first interactions**: Every action exhausts pure accessibility API strategies before falling back to mouse events
- **Structured JSON output**: Machine-readable responses with error codes and recovery hints
- **Works with any app**: Finder, Safari, System Settings, Xcode, Slack — anything with an accessibility tree

## Installation

### npm (recommended)

```bash
npm install -g deskpilot        # downloads prebuilt binary automatically
```

Or without installing:

```bash
npx deskpilot snapshot --app Finder -i
```

### From source

```bash
git clone https://github.com/clawdia-org/deskpilot
cd deskpilot
cargo build --release
cp target/release/deskpilot /usr/local/bin/
```

Requires Rust 1.78+ and macOS 13.0+.

### Permissions

macOS requires Accessibility permission. Grant it in **System Settings > Privacy & Security > Accessibility** by adding your terminal app, or:

```bash
deskpilot permissions --request   # trigger system dialog
```

## Language bindings (FFI)

Every GitHub Release ships a prebuilt C-ABI cdylib alongside the CLI tarballs. Hosts that need in-process calls (Python agents, Swift apps, Go services, Node tools, Ruby scripts, C/C++ code) `dlopen` the dylib and call the functions declared in `deskpilot.h` — no fork-exec per command.

| Platform             | Artifact |
|----------------------|----------|
| macOS arm64          | `deskpilot-ffi-v<ver>-aarch64-apple-darwin.tar.gz` |
| macOS x86_64         | `deskpilot-ffi-v<ver>-x86_64-apple-darwin.tar.gz` |
| Linux x86_64 (glibc) | `deskpilot-ffi-v<ver>-x86_64-unknown-linux-gnu.tar.gz` |
| Linux arm64  (glibc) | `deskpilot-ffi-v<ver>-aarch64-unknown-linux-gnu.tar.gz` |
| Windows x86_64 (MSVC)| `deskpilot-ffi-v<ver>-x86_64-pc-windows-msvc.zip` |

Each archive contains `lib/libdeskpilot_ffi.{dylib,so,dll}`, `include/deskpilot.h`, `LICENSE`, and a short README. Verify the download with the release's `checksums.txt`:

```bash
shasum -a 256 -c checksums.txt
gh attestation verify deskpilot-ffi-v*.tar.gz --repo clawdia-org/deskpilot   # Sigstore provenance
```

Minimal Python round-trip:

```python
import ctypes
lib = ctypes.CDLL("./lib/libdeskpilot_ffi.dylib")
lib.ad_adapter_create.restype = ctypes.c_void_p
adapter = lib.ad_adapter_create()
# ... call ad_list_apps / ad_get_tree / ad_execute_action, see docs below
lib.ad_adapter_destroy(adapter)
```

Full consumer guide — error-handling contract, ownership rules, threading constraints, every entrypoint with Safety docs: [`skills/deskpilot-ffi/`](skills/deskpilot-ffi/).

## Core Workflow for AI

For dense apps (Slack, VS Code, Notion), use **progressive skeleton traversal** to minimize token usage:

```bash
# 1. Shallow overview — depth-3 map, truncated containers show children_count
deskpilot snapshot --skeleton --app Slack -i --compact

# 2. Drill into a region of interest (named containers get refs as drill targets)
deskpilot snapshot --root @e3 -i --compact

# 3. Act on an element found in the drill-down
deskpilot click @e12

# 4. Re-drill the same region to verify the state change
deskpilot snapshot --root @e3 -i --compact
```

For simple apps, a full snapshot is fine:

```bash
deskpilot snapshot --app Finder -i   # get interactive elements with refs
deskpilot click @e3                  # click a button by ref
deskpilot type @e5 "quarterly report"  # type into a text field
deskpilot press cmd+s               # keyboard shortcut
deskpilot snapshot -i               # re-observe after UI changes
```

```
Agent loop:  snapshot → decide → act → snapshot → decide → act → ...
```

## Commands

### Observation

```bash
deskpilot snapshot --app Safari -i           # accessibility tree with refs
deskpilot snapshot --surface menu            # capture open menu
deskpilot screenshot --app Finder            # PNG screenshot
deskpilot find --role button --app TextEdit  # search by role, name, value, text
deskpilot get @e3 value                      # read element property
deskpilot is @e7 checked                     # check boolean state
deskpilot list-surfaces --app Notes          # list menus, sheets, popovers, alerts
```

### Interaction

```bash
deskpilot click @e3                  # smart AX-first click (15-step chain)
deskpilot double-click @e3           # open files, select words
deskpilot triple-click @e3           # select lines/paragraphs
deskpilot right-click @e3            # context menu (returns menu tree inline)
deskpilot type @e5 "hello world"     # type text into element
deskpilot set-value @e5 "new value"  # set value directly via AX
deskpilot clear @e5                  # clear element value
deskpilot focus @e5                  # set keyboard focus
deskpilot select @e9 "Option B"      # select option in dropdown/list
deskpilot toggle @e12                # flip checkbox or switch
deskpilot check @e12                 # idempotent check
deskpilot uncheck @e12               # idempotent uncheck
deskpilot expand @e15                # expand disclosure/tree item
deskpilot collapse @e15              # collapse disclosure/tree item
deskpilot scroll @e1 down 3          # scroll (AX-first, 10-step chain)
deskpilot scroll-to @e20             # scroll element into view
```

### Keyboard

```bash
deskpilot press cmd+s               # key combo
deskpilot press cmd+shift+z          # multi-modifier
deskpilot press escape               # single key
deskpilot key-down shift             # hold key
deskpilot key-up shift               # release key
```

### Mouse

```bash
deskpilot hover @e3                  # move cursor to element
deskpilot hover --xy 500,300         # move cursor to coordinates
deskpilot drag @e3 --to @e8          # drag between elements
deskpilot drag --xy 100,200 --to-xy 400,200  # drag between coordinates
deskpilot mouse-click --xy 500,300   # click at coordinates
deskpilot mouse-down --xy 500,300    # press at coordinates
deskpilot mouse-up --xy 500,300      # release at coordinates
```

### App & Window Management

```bash
deskpilot launch Safari              # launch app by name
deskpilot launch com.apple.Safari    # launch by bundle ID
deskpilot close-app Safari           # quit app
deskpilot close-app Safari --force   # force quit (SIGKILL)
deskpilot list-apps                  # list running GUI apps
deskpilot list-windows               # list visible windows
deskpilot list-windows --app Finder  # windows for specific app
deskpilot focus-window w-4521        # bring window to front
deskpilot resize-window w-4521 800 600  # resize
deskpilot move-window w-4521 100 100    # move
deskpilot minimize w-4521            # minimize
deskpilot maximize w-4521            # maximize
deskpilot restore w-4521             # restore
```

### Notifications *(macOS only)*

```bash
deskpilot list-notifications                       # list all notifications
deskpilot list-notifications --app "Slack"         # filter by app
deskpilot list-notifications --text "deploy" --limit 5  # filter by text
deskpilot dismiss-notification 1                   # dismiss by index
deskpilot dismiss-all-notifications                # dismiss all
deskpilot dismiss-all-notifications --app "Slack"  # dismiss all from app
deskpilot notification-action 1 --action "Reply"   # click action button
```

### Clipboard

```bash
deskpilot clipboard-get              # read clipboard text
deskpilot clipboard-set "copied"     # write to clipboard
deskpilot clipboard-clear            # clear clipboard
```

### Wait

```bash
deskpilot wait 500                                       # sleep 500ms
deskpilot wait --element @e3 --timeout 5000              # wait for element
deskpilot wait --window "Save" --timeout 10000           # wait for window
deskpilot wait --text "Loading complete" --app Safari    # wait for text
deskpilot wait --menu --timeout 3000                     # wait for menu
```

### Batch

```bash
deskpilot batch '[
  {"command": "click", "args": {"ref_id": "@e2"}},
  {"command": "type", "args": {"ref_id": "@e5", "text": "hello"}},
  {"command": "press", "args": {"combo": "return"}}
]' --stop-on-error
```

### System

```bash
deskpilot status                     # platform, permission state
deskpilot permissions                # check accessibility permission
deskpilot permissions --request      # trigger system dialog
deskpilot version                    # version string
```

## Snapshot Options

```bash
deskpilot snapshot [OPTIONS]
```

| Flag | Default | Description |
|------|---------|-------------|
| `--app <NAME>` | focused app | Filter to a specific application |
| `--window-id <ID>` | - | Filter to a specific window |
| `-i` / `--interactive-only` | off | Only include interactive elements |
| `--compact` | off | Omit empty structural nodes |
| `--include-bounds` | off | Include pixel bounds (x, y, width, height) |
| `--max-depth <N>` | 10 | Maximum tree depth |
| `--skeleton` | off | Shallow 3-level overview; truncated containers show `children_count` and get refs as drill targets |
| `--root <REF>` | - | Start traversal from this ref; merges into existing refmap with scoped invalidation |
| `--surface <TYPE>` | window | `window`, `focused`, `menu`, `menubar`, `sheet`, `popover`, `alert` |

## JSON Output

Every command returns structured JSON:

```json
{
  "version": "1.0",
  "ok": true,
  "command": "click",
  "data": { "action": "click" }
}
```

Errors include machine-readable codes and recovery hints:

```json
{
  "version": "1.0",
  "ok": false,
  "command": "click",
  "error": {
    "code": "STALE_REF",
    "message": "Element at @e7 no longer matches the last snapshot",
    "suggestion": "Run 'snapshot' to refresh refs, then retry"
  }
}
```

### Error Codes

| Code | Meaning |
|------|---------|
| `PERM_DENIED` | Accessibility permission not granted |
| `ELEMENT_NOT_FOUND` | No element matched the ref or query |
| `APP_NOT_FOUND` | Application not running or no windows |
| `STALE_REF` | Ref is from a previous snapshot |
| `ACTION_FAILED` | The OS rejected the action |
| `TIMEOUT` | Wait condition expired |
| `INVALID_ARGS` | Invalid argument values |

### Exit Codes

`0` success, `1` structured error (JSON on stdout), `2` argument parse error.

## Ref System

`snapshot` assigns refs to interactive elements in depth-first order: `@e1`, `@e2`, `@e3`, etc. Refs are valid until the next snapshot replaces them.

Interactive roles that receive refs: `button`, `textfield`, `checkbox`, `link`, `menuitem`, `tab`, `slider`, `combobox`, `treeitem`, `cell`, `radiobutton`, `incrementor`, `menubutton`, `switch`, `colorwell`, `dockitem`.

Static elements (labels, groups, containers) appear in the tree for context but have no ref.

Stale ref recovery:

```
snapshot → act → STALE_REF? → snapshot again → retry
```

## Platform Support

| | macOS | Windows | Linux |
|---|:---:|:---:|:---:|
| Accessibility tree | **Yes** | Planned | Planned |
| Click / type / keyboard | **Yes** | Planned | Planned |
| Mouse input | **Yes** | Planned | Planned |
| Screenshot | **Yes** | Planned | Planned |
| Clipboard | **Yes** | Planned | Planned |
| App & window management | **Yes** | Planned | Planned |
| Notifications | **Yes** | Planned | Planned |

## Development

```bash
cargo build                               # debug build
cargo build --release                     # optimized (<15MB)
cargo test --lib --workspace              # run tests
cargo clippy --all-targets -- -D warnings # lint (must pass with zero warnings)
```

## License

Apache-2.0
