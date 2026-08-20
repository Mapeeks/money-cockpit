# Contributing to Money Cockpit

Thank you for your interest in contributing. Money Cockpit is open to contributions from day one — here's everything you need to know before submitting your first PR.

---

## Ground Rules

- **No AI-generated code.** Every line must be written by a human. This is non-negotiable and will be checked during review.
- **Minimize dependencies.** If your contribution introduces a new dependency, you must justify it explicitly in your PR. Only dependencies with clear added value and no security risk will be accepted.
- **Sign the CLA.** All contributors must sign the [Contributor License Agreement](./CLA.md) before their first PR is merged.

---

## Before You Code — Open an Issue First

**Every contribution must start with an issue.** No exceptions, including small fixes. This avoids duplicate work, misaligned efforts, and PRs that go in the wrong direction.

### Issue Labels

**Type** (required — pick one):

| Label         | Use for                                                                        |
| ------------- | ------------------------------------------------------------------------------ |
| `bug`         | Something is broken                                                            |
| `feature`     | A new capability                                                               |
| `enhancement` | Improving an existing feature                                                  |
| `chore`       | Technical task with no functional impact (dependency update, refactor, config) |
| `docs`        | Documentation only                                                             |
| `security`    | Security-related concern                                                       |

**Scope** (optional):

`core` · `storage` · `ci-cd` · `feature/accounts` · `feature/transactions` · `feature/dashboard`

Scopes will expand as the project grows. If none fits, leave it blank.

**Status** (managed by maintainers):

| Label          | Meaning                                               |
| -------------- | ----------------------------------------------------- |
| `needs-triage` | Opened, not yet reviewed                              |
| `in-progress`  | Someone is actively working on it                     |
| `wontfix`      | Valid but out of scope or intentionally not addressed |
| `duplicate`    | Already covered by another issue                      |

---

## Workflow

1. **Open an issue** and wait for it to move to `in-progress` before starting work.
2. **Fork the repository** and create a branch from `main`.
3. **Name your branch** clearly: `fix/dashboard-balance`, `feat/add-transaction`, `chore/update-rusqlite`.
4. **Write clean, human code.** No generated code, no unnecessary abstractions.
5. **Open a Pull Request** against `main` with a clear description linking the related issue (`Closes #42`).
6. **Address review comments.** The maintainer will request changes if needed — you are expected to apply them yourself.
7. **Once approved and merged**, the linked issue is closed automatically.

---

## Pull Request Checklist

Before submitting, make sure:

- [ ] An issue exists and is linked in the PR description
- [ ] The code is human-written
- [ ] No new dependency is added without justification
- [ ] The CLA has been signed
- [ ] The project builds without errors (`cargo tauri dev`)

---

## Code Style

Follow standard Rust conventions. Run `cargo fmt` and `cargo clippy` before pushing.

---

## Questions?

Open an issue with the `docs` label or reach out via [mapeeks.com](https://mapeeks.com).
