# Interaction Commands

Commands for modifying UI state — clicking, typing, selecting, scrolling, and input synthesis.

## Click Actions

All click commands use a smart activation chain (AX-first) that tries accessibility actions before falling back to coordinate-based clicks.

### click
```bash
deskpilot click @e5
```
Primary activation. Tries AXPress > AXConfirm > AXOpen > AXPick > child activation > focus+activate > coordinate click.

### double-click
```bash
deskpilot double-click @e3
```
Tries AXOpen first, then two smart activations with 50ms gap, then CGEvent double-click.

### triple-click
```bash
deskpilot triple-click @e2
```
Three smart activations with 30ms gaps, then CGEvent triple-click. Useful for select-all in text fields.

### right-click
```bash
deskpilot right-click @e5
```
Opens context menu. Tries AXShowMenu > focus+AXShowMenu > parent/child AXShowMenu > coordinate right-click. Use `wait --menu` after to capture the menu, then `snapshot --surface menu` to read it.

## Text Input

### type
```bash
deskpilot type @e2 "hello@example.com"
deskpilot type @e2 "multi line\ntext"
```
Focuses the element then types each character via keyboard synthesis. Handles special characters.

### set-value
```bash
deskpilot set-value @e2 "new value"
```
Sets the value directly via the AX value attribute. Faster than `type` but may not trigger all UI callbacks. Use for text fields, text areas, and sliders.

### clear
```bash
deskpilot clear @e2
```
Clears the element's value to an empty string. Equivalent to `set-value @e2 ""`.

### focus
```bash
deskpilot focus @e2
```
Sets keyboard focus on the element without clicking it.

## Selection & Toggle

### select
```bash
deskpilot select @e4 "Option B"
```
Selects an option in a list, dropdown, or combobox by its display text.

### toggle
```bash
deskpilot toggle @e6
```
Toggles a checkbox or switch to the opposite state.

### check
```bash
deskpilot check @e6
```
Sets a checkbox or switch to the checked/on state. Idempotent — does nothing if already checked.

### uncheck
```bash
deskpilot uncheck @e6
```
Sets a checkbox or switch to the unchecked/off state. Idempotent.

## Expand & Collapse

### expand
```bash
deskpilot expand @e7
```
Expands a disclosure triangle, tree item, or accordion.

### collapse
```bash
deskpilot collapse @e7
```
Collapses an expanded disclosure/tree item.

## Scrolling

### scroll
```bash
deskpilot scroll @e1 --direction down --amount 3
deskpilot scroll @e1 --direction up --amount 5
deskpilot scroll @e1 --direction left --amount 2
deskpilot scroll @e1 --direction right --amount 2
```

| Flag | Default | Description |
|------|---------|-------------|
| `--direction` | down | `up`, `down`, `left`, `right` |
| `--amount` | 3 | Number of scroll units |

### scroll-to
```bash
deskpilot scroll-to @e8
```
Scrolls the element into the visible area of its scroll container.

## Keyboard

### press
```bash
deskpilot press return
deskpilot press escape
deskpilot press cmd+c
deskpilot press cmd+shift+z
deskpilot press shift+tab
deskpilot press f5
deskpilot press cmd+a --app "TextEdit"
```

| Flag | Description |
|------|-------------|
| `--app` | Target application (focuses app before pressing) |

**Key names:** `return`, `escape`, `tab`, `space`, `delete`, `up`, `down`, `left`, `right`, `f1`-`f12`
**Modifiers:** `cmd`, `ctrl`, `alt`, `shift` — combine with `+`

### key-down
```bash
deskpilot key-down shift
```
Holds a key or modifier down. Must be paired with `key-up`.

### key-up
```bash
deskpilot key-up shift
```
Releases a held key or modifier.

## Mouse

### hover
```bash
deskpilot hover @e5
deskpilot hover --xy 500,300
deskpilot hover @e5 --duration 2000
```
Moves cursor to element center or absolute coordinates. Optional `--duration` holds position for N ms.

### drag
```bash
deskpilot drag --from @e1 --to @e5
deskpilot drag --from-xy 100,200 --to-xy 400,500
deskpilot drag --from @e1 --to-xy 400,500 --duration 500
```

| Flag | Description |
|------|-------------|
| `--from` | Source element ref |
| `--from-xy` | Source coordinates as `x,y` |
| `--to` | Destination element ref |
| `--to-xy` | Destination coordinates as `x,y` |
| `--duration` | Drag duration in milliseconds |

Can mix ref and coordinate sources (e.g., `--from @e1 --to-xy 400,500`).

### mouse-move
```bash
deskpilot mouse-move --xy 500,300
```
Moves cursor to absolute screen coordinates.

### mouse-click
```bash
deskpilot mouse-click --xy 500,300
deskpilot mouse-click --xy 500,300 --button right
deskpilot mouse-click --xy 500,300 --count 2
```

| Flag | Default | Description |
|------|---------|-------------|
| `--xy` | (required) | Coordinates as `x,y` |
| `--button` | left | `left`, `right`, `middle` |
| `--count` | 1 | Number of clicks |

### mouse-down / mouse-up
```bash
deskpilot mouse-down --xy 100,200
deskpilot mouse-up --xy 300,400
```
Low-level press/release for custom drag or hold interactions.

| Flag | Default | Description |
|------|---------|-------------|
| `--xy` | (required) | Coordinates as `x,y` |
| `--button` | left | `left`, `right`, `middle` |

## Choosing the Right Command

| Goal | Preferred | Alternative |
|------|-----------|-------------|
| Click a button | `click @ref` | `mouse-click --xy` if AX fails |
| Fill a text field | `type @ref "text"` | `set-value @ref "text"` for direct set |
| Clear then type | `clear @ref` then `type @ref "new"` | `triple-click @ref` then `type @ref "new"` |
| Toggle a checkbox | `check @ref` / `uncheck @ref` | `toggle @ref` if you don't know current state |
| Open context menu | `right-click @ref` then `wait --menu` | `mouse-click --xy --button right` |
| Select dropdown option | `select @ref "Option"` | `click @ref` then `find` the option |
| Navigate a form | `press tab` between fields | `focus @ref` to jump directly |
| Copy text | `press cmd+c --app "App"` | `clipboard-set` to set directly |
| Scroll to find elements | `scroll @ref --direction down` | `scroll-to @ref` if you have the ref |
