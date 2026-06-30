# TASK.md

# Metadata

```yaml
task: 1.2

title: "XML Parser"

milestone: "Milestone 1"

epic: "Epic 1.2"

status: planned

implementation: interface-only

scope:

  - compiler

  - documentation

affected_crates:

  - fhir-parser

  - fhir-ir

  - fhirc

references:

  - ARCHITECTURE.md
  - STYLE.md
  - PATTERNS.md
  - INVARIANTS.md
  - IR.md
  - ROADMAP.md
  - CURRENT.md

tests:

  - unit
  - integration
  - snapshot
```

---

# Background

Following completion of:

- Milestone 0 (Repository Foundation)
- Milestone 0.1 (Workspace, CI, formatting, linting, license)
- Milestone 0.2 (Golden testing framework)
- Milestone 0.3 (Intermediate Representation Definition)
- Milestone 0.4 (Compiler CLI)
- Epic 1.1 (Specification loader) — in progress

we will now implement:

- Epic 1.2 — XML parser

This is the first Source Layer compiler component.

It consumes raw FHIR XML artifacts (StructureDefinition, ValueSet, CodeSystem) and produces the canonical Initial IR required by all downstream semantic passes.

---

# Objective

Define and implement the public interface of the FHIR XML parser.

The parser converts official FHIR XML specification files into the project's Intermediate Representation.

Only interface definitions and tests are implemented during this task.

Actual parsing logic is deferred to a subsequent task.

---

# Scope

This task implements only:

- `fhir-parser` crate public API surface (trait definitions and type stubs)
- XML token stream interface types
- Error type enumeration for parse-time failures
- Failing unit and integration tests covering the expected parser contract

The following are explicitly OUT OF SCOPE:

- XML parsing logic or any `quick-xml` integration
- Semantic compiler passes (reference resolution, flattening, choice resolution, etc.)
- Rust code generation
- CLI commands beyond stubs reserving parse entry points
- Runtime libraries or server code

Any newly discovered work should be added to ROADMAP.md rather than implemented.

---

# Functional Requirements

- Define `Parse` trait producing deterministic Initial IR output.
- Define `ParseError` enum covering structured XML parse failure modes required by INV-002 (no panics on invalid input).
- Preserve per-invariant behavior:
  - INV-001: Parser must be deterministic (output ordering fixed by FHIR source).
  - INV-003: Parser produces Initial IR only; never emits Rust directly.
  - INV-004: Backends consume only IR — parser stops at Initial IR.
  - INV-005: Parsing is a compiler pass (IR → IR).
  - INV-014: Output contains no timestamps or machine-specific values.
  - INV-015: Generated code formatting invariants are not yet relevant; format the parser source itself with `cargo fmt` / `cargo clippy`.
- Provide CLI stub for `fhirc parse` that returns `NotImplemented` until parser logic is implemented.

---

# Technical Requirements

## Rust

Implement idiomatic modern Rust.

Prefer:

- composition
- traits over inheritance (ADR-007)
- semantic types (`Id`, `Uri`, `Code`) per INV-010
- zero-cost abstractions

Avoid object-oriented designs unless required by the FHIR specification.

## Architecture

All implementation MUST conform to:

- ARCHITECTURE.md
- STYLE.md
- PATTERNS.md
- INVARIANTS.md

Compiler work must additionally conform to:

- IR.md

## Parser Layer Position

The parser is the Source Layer component according to ARCHITECTURE.md.

It sits at:

```
FHIR XML
   │
   ▼
Parse (Initial IR)
```

Parser output is consumed by semantic compiler passes.

Parser never bypasses the IR (INV-003, INV-004, INV-005).

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
| ARCHITECTURE.md | Parse → Transform → Emit architecture; Source Layer role of parser |
| STYLE.md | Rust style, error handling, semantic types, module layout |
| PATTERNS.md | Resource structure, trait design, golden tests, module layout |
| INVARIANTS.md | INV-001 through INV-005, INV-010, INV-014, INV-015, INV-020 |
| IR.md | Initial IR contract, semantic passes ordering |
| ROADMAP.md | Epic placement within Milestone 1 |
| CURRENT.md | Confirms Epic 1.1 in progress and Epic 1.2 is next |

Additional references:

- HL7 FHIR R4 / R5 StructureDefinition XML specification
- Rust `quick-xml` crate (parsing engine for next task, not required now)

---

# Notes

Keep the implementation narrowly focused.

Do not design parser internals beyond the public trait boundary.

Prefer incremental progress over ambitious refactoring.

If architectural improvements are identified, record them in ROADMAP.md or an ADR rather than expanding the scope of this task.
