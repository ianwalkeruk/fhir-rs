# TASK.md

This file is generated from TASK.template.md.

It defines exactly one implementation task.

Once completed it should be deleted.

---

# Metadata

```yaml
task: {task-number}

title: "{task title}"

milestone: "{milestone}"

epic: "{epic}"

status: planned

implementation: interface-only

scope:

  - compiler
  - runtime
  - generated
  - documentation

affected_crates:

  - crate-name

references:

  - ARCHITECTURE.md
  - STYLE.md

tests:

  - unit
  - property
  - integration
  - snapshot
```

---

# Background

{Brief description of recent work.}

Following completion of:

{previous roadmap item}

we will now implement:

{current roadmap item}

---

# Objective

{One paragraph describing the purpose of this task.}

---

# Scope

This task implements only:

- ...
- ...
- ...

The following are explicitly OUT OF SCOPE:

- ...
- ...
- ...

Any newly discovered work should be added to ROADMAP.md rather than implemented.

---

# Functional Requirements

- Requirement 1
- Requirement 2
- Requirement 3

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

---

## Architecture

All implementation MUST conform to:

- ARCHITECTURE.md
- STYLE.md
- PATTERNS.md
- INVARIANTS.md

Compiler work must additionally conform to:

- IR.md

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
- integration tests
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
| {document} | {reason} |

Additional references:

- {FHIR specification section}
- {RFC}
- {GitHub issue}

---

# Notes

Keep the implementation narrowly focused.

Do not redesign unrelated code.

Prefer incremental progress over ambitious refactoring.

If architectural improvements are identified, record them in ROADMAP.md or an ADR rather than expanding the scope of this task.