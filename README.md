# Stilla

Small tray. Big status.

> Drop a script. Read it in your menubar.

<table>
  <tbody>
    <tr>
      <td><img src="https://github.com/user-attachments/assets/c5d7f907-b569-4992-b99c-bf7c8a4ba638" /></td>
      <td><img src="https://github.com/user-attachments/assets/28018280-f8c3-4a76-9330-860dba9fd684" /></td>
    </tr>
  </tbody>
</table>

Stilla is a cross-platform menubar/system tray utility that runs your shell scripts on a timer/schedule and shows their output right in the system tray.

Write any script, name it with an interval like `status.30s.sh`, and Stilla parses its JSON output into a live tray icon title and a dynamic, clickable menu. No windows. No config files. Just your menubar.

Because the status you need to see always lives one click too deep.

## Why it feels good

- **Live** — scripts re-run on a timer; the tray updates automatically
- **Portable** — one binary for macOS, Linux, and Windows
- **Scriptable** — any shell script that speaks JSON works
- **Local-first** — no servers, no daemons, local-only, privacy-respecting
- **Simple** — drop a `.sh` file in `~/.stilla/` and you're done

## Quick start

1. Download the latest release or build from source
2. Create a script in `~/.stilla/` (e.g. `status.30s.sh`)
3. Have it print a JSON object with `title` and `items`
4. Run `stilla` — the tray icon shows your title, right-click for the menu

```bash
#!/bin/sh

echo '{"title": "Tray title","items": [{"item_type": "text", "title": "Hello ✅"},{"item_type": "link", "title": "Link", "path": "https://example.com"},{"item_type": "divider"},{"item_type": "submenu", "title": "More", "items": [{"item_type": "link", "title": "Nested", "path": "https://example.com/nested"}]}]}'
```

> Tip: append an `n` to the interval (e.g. `alerts.5m.sh` → `alerts.5mn.sh`) to get desktop notifications when items change.

## Technical summary

| Area       | Details                                                         |
| ---------- | --------------------------------------------------------------- |
| UI         | System tray via `tray-icon`, menu rebuilt on output change      |
| Runtime    | `tao` event loop, `tokio` async script executor                 |
| Scripts    | `sh <script>` with 30s timeout, 256 KB output cap               |
| Output     | JSON with `title`, `items` (link / text / divider / submenu)    |
| Scheduling | Interval encoded in filename: `30s`, `5m`, `2h`, `1d`           |
| Notify     | Optional `n` flag in filename triggers `notify-rust` on changes |

### Repo layout

- `src/main.rs` — event loop, tray setup, app lifecycle
- `src/plugin/` — plugin manager, script runner, menu builder, output hashing
- `src/utils/` — path helpers, process runner, time parsing, notifications
- `.stilla/` — example plugin scripts

### Build

```bash
cargo build --release
```
