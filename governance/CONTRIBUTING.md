# Contributing to ADR

Thank you for your interest in ADR – Agent-Oriented Declarative Runtime.

## How to Contribute

### Specification Feedback
Open a GitHub Issue with the label `spec-feedback`.
Describe clearly which section you are commenting on and why.

### Bug Reports
Open a GitHub Issue with the label `bug`.
Include a concrete description and, if possible, a minimal example.

### Code Contributions (Phase B onwards)
- Fork the repository
- Create a branch: `git checkout -b my-change`
- Make your changes
- Run tests: `cargo test --all`
- Open a Pull Request against the `main` branch

All contributions must respect the ADR Scope Disclaimer (see [SCOPE.md](../SCOPE.md)):
ADR does not replace deterministic low-level safety loops or hardware interlocks.

## Versioning

We use Semantic Versioning: `0.5.0 → 0.6.0 → 0.7.0`
Dates are recorded in [CHANGELOG.md](../CHANGELOG.md), not in version numbers.

## Crates.io Publishing

Deferred until the Layer 1 skeleton (adr-core) is complete.
Currently workspace-internal only.

---

*ADR Runtime Contributors – February 2026*
