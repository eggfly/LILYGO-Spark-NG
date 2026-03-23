# Feature Alignment: Electron vs Rust GPUI (NT)

## Status Legend
- ✅ = Done (UI + Logic)
- 🟡 = UI done, logic missing
- 🔴 = Not implemented
- 📐 = UI partially done

## Overall Progress

| Category | Electron Features | NT Implemented | Alignment |
|----------|:-:|:-:|:-:|
| Navigation & Layout | 7 | 6 | 86% |
| Pages (8 total) | 8 | 8 | 100% (structure) |
| Theme System | 7 | 5 | 71% |
| i18n | 5 | 4 | 80% |
| Animations & Effects | 7 | 1 | 14% |
| Data & Backend | 7 | 1 | 14% |
| Serial/Flash | 7 | 0 | 0% |
| **Overall** | | | **~55% UI, ~35% functional** |

---

## Page-by-Page Comparison

### 1. Sidebar Navigation

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| 8 nav items with icons | ✅ | ✅ | ✅ |
| Logo + "Made with AI & Love" | ✅ | ✅ | ✅ |
| Active indicator (left bar) | ✅ | ✅ | ✅ |
| Active bg glow/shadow | ✅ | ✅ | ✅ |
| GitHub login button | ✅ | 🟡 | UI only |
| User avatar + profile | ✅ | 🔴 | Missing |
| Upload button (auth) | ✅ | 🔴 | Missing |
| Version display | ✅ | ✅ | ✅ |
| Update progress bar | ✅ | 🔴 | Missing |
| Download count badge | ✅ | 🔴 | Missing |
| Hover tooltips (portal) | ✅ | 🔴 | Missing |
| Glass effect sidebar | ✅ | ✅ | ✅ |
| macOS traffic light offset | ✅ | ✅ | ✅ |
| Windows custom titlebar | ✅ | ✅ | ✅ |

### 2. Discovery Page

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| Page header + subtitle | ✅ | ✅ | ✅ |
| News card grid | ✅ | ✅ | ✅ |
| RSS feed fetching (Hackaday, CNX, Adafruit) | ✅ | 🔴 | Hardcoded |
| Card: image, title, summary, date, tags | ✅ | 🟡 | No real images |
| Refresh button | ✅ | 🟡 | UI only |
| Click to open article | ✅ | 🔴 | Missing |
| Loading/error states | ✅ | 🔴 | Missing |
| Mock data fallback | ✅ | 🟡 | Always mock |

### 3. Firmware Center

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| 3-column layout (series/products/firmware) | ✅ | 📐 | 2-col in NT |
| Product series grouping (collapsible) | ✅ | 🔴 | Flat list |
| Product images | ✅ | 🔴 | Emoji placeholder |
| Search bar (functional) | ✅ | 🟡 | UI only, no filter |
| "Only with firmware" checkbox | ✅ | ✅ | ✅ |
| Product count badge | ✅ | ✅ | ✅ |
| Product selection + highlight | ✅ | ✅ | ✅ |
| Firmware list with badges | ✅ | ✅ | ✅ |
| Download button | ✅ | 🟡 | UI only |
| Burn button | ✅ | 🔴 | Missing |
| Save As / Delete / Analyze buttons | ✅ | 🔴 | Missing |
| Download progress tracking | ✅ | 🔴 | Missing |
| OSS/GitHub source links | ✅ | 🔴 | Missing |
| File size + compression display | ✅ | 📐 | Size only |

### 4. Firmware Lab

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| 4-tab layout (Burner/Dumper/Analyzer/Editor) | ✅ | 📐 | Tabs shown, no switch |
| Burner: Basic mode | ✅ | 🟡 | UI layout only |
| Burner: Advanced mode | ✅ | 🔴 | Missing |
| Port detection + selection | ✅ | 🔴 | Static dropdown |
| Baud rate selection | ✅ | 🟡 | UI only |
| esptool integration (native + JS) | ✅ | 🔴 | Missing |
| xterm terminal | ✅ | 🔴 | Dark div placeholder |
| Flash progress tracking | ✅ | 🔴 | Missing |
| Dumper: device info detection | ✅ | 🔴 | Missing |
| Analyzer: binary firmware analysis | ✅ | 🔴 | Missing |
| Partition Editor: visual editor | ✅ | 🔴 | Missing |
| File drag & drop | ✅ | 🔴 | Missing |

### 5. Serial Tools

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| Port selector | ✅ | 🟡 | Static UI |
| Baud rate (9600-2000000) | ✅ | 🟡 | Shows 115200 only |
| Connect/Disconnect | ✅ | 🟡 | Button only |
| Terminal output (xterm) | ✅ | 🟡 | Mock text |
| Command input + Send | ✅ | 🟡 | UI only |
| Auto-scroll toggle | ✅ | 🟡 | UI only |
| Clear button | ✅ | 🟡 | UI only |
| Auto-warnings detection | ✅ | 🔴 | Missing |
| 5000-line buffer | ✅ | 🔴 | Missing |
| Port refresh | ✅ | 🔴 | Missing |

### 6. Embedded Tools (12 calculators)

| Tool | Electron | NT | Status |
|------|:--------:|:--:|:------:|
| Resistor Color Code | ✅ full | 📐 | Mock display only |
| Image Converter | ✅ | 🔴 | Missing |
| Voltage Divider | ✅ | 🔴 | Missing |
| RC Time Constant | ✅ | 🔴 | Missing |
| Ohm's Law | ✅ | 🔴 | Missing |
| 555 Timer | ✅ | 🔴 | Missing |
| SMD Resistor | ✅ | 🔴 | Missing |
| LED Resistor | ✅ | 🔴 | Missing |
| Battery Life | ✅ | 🔴 | Missing |
| ESP32 Power Mode | ✅ | 🔴 | Missing |
| Series/Parallel | ✅ | 🔴 | Missing |
| Circuit Schematic | ✅ | 🔴 | Missing |

### 7. Community

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| 6 link cards | ✅ | ✅ | ✅ |
| Gradient left bar accent | ✅ | ✅ | ✅ |
| Icon with gradient bg | ✅ | ✅ | ✅ |
| Hover effects | ✅ | ✅ | ✅ |
| Click to open links | ✅ | 🔴 | Missing |

### 8. Spark Lab

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| Sparkling List tab | ✅ | ✅ | ✅ |
| Guide tab | ✅ | 🔴 | Missing |
| 4 categories with items | ✅ | 📐 | 2 categories |
| Status badges (Shipped/Planned/Spark) | ✅ | ✅ | ✅ |
| Progress bar | ✅ | ✅ | ✅ |
| Expandable/collapsible categories | ✅ | 🔴 | Always expanded |

### 9. Settings

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| Language selector | ✅ | ✅ | ✅ (UI + logic) |
| Theme preference | ✅ | 🟡 | UI only, not applied |
| Accent color picker | ✅ | ✅ | ✅ |
| Accent mode (rotating/fixed) | ✅ | 🟡 | UI only |
| Glass effect toggle | ✅ | 🟡 | UI only |
| Sound toggle | ✅ | 🟡 | UI only |
| Flash celebration style | ✅ | 🟡 | UI only |
| Link open mode | ✅ | 🟡 | UI only |
| Easter eggs section | ✅ | ✅ | ✅ |
| Advanced settings (collapsible) | ✅ | ✅ | ✅ |
| Developer mode | ✅ | 🟡 | UI only |
| Canary channel | ✅ | 🟡 | UI only |
| Manifest file override | ✅ | 🟡 | UI only |
| Cache management | ✅ | 🟡 | UI only |
| Network proxy | ✅ | 🔴 | Missing |
| Live log viewer | ✅ | 🔴 | Missing |
| Feedback form | ✅ | 🔴 | "Coming soon" |

---

## Animations & Effects

| Feature | Electron | NT | Status |
|---------|:--------:|:--:|:------:|
| Page fade-in transition | ✅ | 🔴 | Missing |
| Fireworks canvas | ✅ | 🔴 | Missing |
| Flash celebration overlay (6 styles) | ✅ | 🔴 | Missing |
| Hacker easter egg (matrix style) | ✅ | 🔴 | Missing |
| Konami code detection | ✅ | 🔴 | Missing |
| Device toast (slide-in) | ✅ | 🔴 | Missing |
| Pulse animation (download badge) | ✅ | 🔴 | Missing |
| Icon scale on hover | ✅ | 🔴 | Missing |
| Glass mesh gradient bg | ✅ | 📐 | Linear gradient only |

---

## Priority Roadmap

### P0 - Quick Wins (UI polish, no backend needed)
1. ~~Firmware center search functionality~~ → wire up search_query filter
2. ~~Accent color actually applied to sidebar/theme~~ → wire accent to PRIMARY
3. ~~Theme mode switch (dark/light) applied~~ → change bg/text colors
4. ~~Page fade-in animation~~ → GPUI AnimationExt
5. ~~Firmware lab tab switching~~ → add active_lab_tab state
6. ~~Spark Lab: add missing 2 categories~~ → Embedded Calculators + Creative

### P1 - Interactive features
7. Embedded tools: implement Ohm's Law calculator (simplest)
8. Embedded tools: implement Resistor Color Code (interactive)
9. Community page: open-external links
10. Settings: persist & apply theme/glass/accent

### P2 - Backend integration
11. Discovery: fetch RSS feeds via reqwest
12. Firmware center: download firmware files
13. Serial tools: serial port detection (serialport crate)
14. Firmware lab: esptool integration

### P3 - Effects & Polish
15. Konami code easter egg
16. Flash celebration animations
17. Device toast notifications
18. Fireworks canvas

---

## File Structure (NT)

```
src/
├── main.rs          # Entry point, window, keybindings
├── app.rs           # SparkApp model, state, Render impl
├── sidebar.rs       # Sidebar navigation component
├── theme.rs         # Color constants, glass helpers
├── i18n.rs          # Translation system (4 languages, 170+ keys)
├── manifest.rs      # Firmware manifest data model
└── pages/
    ├── mod.rs           # Page enum, routing
    ├── discovery.rs     # News feed (hardcoded)
    ├── firmware_center.rs # Product + firmware browser
    ├── firmware_lab.rs  # Flash/dump/analyze tools
    ├── serial_tools.rs  # Serial monitor
    ├── embedded_tools.rs # Calculator tools
    ├── community.rs     # Community links
    ├── spark_lab.rs     # Roadmap + guide
    └── settings.rs      # All settings
```
