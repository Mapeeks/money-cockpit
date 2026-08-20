# 💰 Money Cockpit

> Open source, human-built personal finance manager. Local-first, privacy-focused, built with Rust & Tauri.

**Made in France 🇫🇷 — by [Mapeeks](https://mapeeks.com)**

---

## Why Money Cockpit?

Most personal finance apps either send your data to a server, lock you into a subscription, or depend on a single maintainer who could stop tomorrow. Money Cockpit is different:

- **100% local** — your data never leaves your machine. One folder, fully portable.
- **Open contributors** — anyone can submit a PR. No single point of failure.
- **Human-built** — no AI-generated code. Every line is written and reviewed by humans, for full project control and auditability.
- **Minimal dependencies** — only what genuinely adds value and doesn't compromise security.

---

## Features (v1 — MVP)

- Create and manage accounts
- Add transactions manually (income & expenses)
- Dashboard: balances, income vs. expenses per month

That's it. On purpose.

> Upcoming (not yet planned): CSV/OFX/QIF import, transaction categories, recurring operations, automatic categorization post-import.

---

## Tech Stack

| Layer            | Choice                                      | Reason                             |
| ---------------- | ------------------------------------------- | ---------------------------------- |
| Language         | Rust                                        | Performance, safety, no runtime    |
| UI               | Tauri                                       | Cross-platform, minimal footprint  |
| Storage          | SQLite via `rusqlite`                       | Local file, auditable, zero server |
| Target platforms | Windows, macOS, Linux (Android & iOS later) |                                    |

---

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (for Tauri CLI)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Build & Run

```bash
git clone https://github.com/Mapeeks/money-cockpit.git
cd money-cockpit
cargo tauri dev
```

### Build for production

```bash
cargo tauri build
```

---

## Data & Privacy

All data is stored in a single local SQLite file. No telemetry, no analytics, no network calls. You own your data entirely — back it up, move it, delete it as you see fit.

---

## Contributing

Money Cockpit is open to contributions from day one. Read [CONTRIBUTING.md](./CONTRIBUTING.md) before submitting a PR.

**Ground rules:**

- No AI-generated code. Every contribution must be human-written.
- Minimize dependencies — if you add one, justify it in your PR.
- All contributors must sign the [Contributor License Agreement (CLA)](./CLA.md) before their first PR is merged. This allows Money Cockpit to be distributed under dual licensing while keeping the community version free and open.

---

## License

Money Cockpit is released under the **GNU General Public License v3.0**.  
See [LICENSE](./LICENSE) for full terms.

A commercial license is available for organizations that wish to integrate Money Cockpit without the GPL obligations. Contact: [maxime@mapeeks.com](mailto:contact@mapeeks.com)

---

## Maintainer

Built and maintained by **Maxime Oriol** — [mapeeks.com](https://mapeeks.com)

---

_Money Cockpit is proudly built in France 🇫🇷 and open to contributors worldwide._
