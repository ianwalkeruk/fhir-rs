# TASK.md

This file is generated from TASK.template.md.

It defines exactly one implementation task.

Once completed it should be deleted.

---

# Metadata

```yaml
task: 0.2

title: "Golden testing framework"

milestone: "Milestone 0"

epic: "Epic 0.2"

status: planned

implementation: interface-only

scope:

  - compiler

affected_crates:

  - compiler/fhir-ir

  - compiler/fhirc

references:

  - ARCHITECTURE.md

  - STYLE.md

tests:

  - unit

  - property

  - snapshot
```

---

# Background

Following completion of:

- Task 0.1 (Workspace Foundation)
- Task 0.3 (IR Schema)
- Task 0.4 (Compiler CLI)

we will now implement:

Epic 0.2 - Golden testing framework

The IR schema has been implemented as Rust structures with deterministic YAML serialization. The CLI provides validation and canonicalization commands. However, a comprehensive golden testing framework for compiler output regression testing is still needed.

---

# Objective

Implement a golden testing framework that enables deterministic regression testing of the compiler's IR output. This framework will verify that compiler passes produce stable, reproducible output and will serve as the foundation for testing all future compiler functionality.

---

# Scope

This task implements only:

- Golden test infrastructure in `compiler/fhir-ir`
- Integration with existing IR types
- Determinism verification utilities

The following are explicitly OUT OF SCOPE:

- Compiler harness implementation
- Parser integration
- Code generation tests

Any newly discovered work should be added to ROADMAP.md rather than implemented.

---

# Functional Requirements

- Golden tests verify deterministic YAML output
- Golden tests verify schema stability
- Golden tests verify serialisation round-tripping
- Test infrastructure supports arbitrary IR snapshots

---

# Technical Requirements

## Rust

Implement idiomatic modern Rust.

Prefer:

- composition
- traits
- semantic types
- zero-cost abstractions

Avoid object-oriented designs unless required by the FHIR specification.

## Architecture

All implementation MUST conform to:

- ARCHITECTURE.md
- STYLE.md
- PATTERNS.md
- INVARIANTS.md

---

## Test-Driven Development

This task follows an interface-first TDD workflow.

During the first implementation phase:

- define public interfaces
- create failing tests
- do not implement behaviour

Tests should include an appropriate mixture of:

- unit tests
- property tests using proptest
- snapshot / golden tests

---

# Definition of Done

- [ ] Public API defined
- [ ] Tests written
- [ ] Interfaces compile
- [ ] cargo fmt passes
- [ ] cargo clippy passes
- [ ] cargo test passes
- [ ] Documentation updated
- [ ] No architectural invariants violated

---

# References

| Document | Purpose |
|----------|---------|
| IR.md | IR schema and structure definitions |
| INVARIANTS.md | Deterministic output requirements |

Additional references:

- PATTERNS.md

---

# Notes

Keep the implementation narrowly focused.

Do not redesign unrelated code.

Prefer incremental progress over ambitious refactoring.

If architectural improvements are identified, record them in ROADMAP.md or an ADR rather than expanding the scope of this task.