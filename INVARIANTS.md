# INVARIANTS.md

> Architectural invariants for the FHIR compiler.
>
> An invariant is a property that must always be true.
>
> Violating an invariant is considered a compiler defect.
>
> These rules are intentionally strict.

---

# Compiler Invariants

## INV-001

The compiler is deterministic.

Given identical specification inputs, the generated Rust must be byte-for-byte identical.

---

## INV-002

The compiler is pure.

Compiler passes must not perform network access or depend upon external state.

Inputs determine outputs.

---

## INV-003

Every generated Rust type originates from exactly one IR node.

Generation never occurs directly from XML.

```
FHIR XML

↓

Specification

↓

IR

↓

Rust
```

---

## INV-004

The IR is the single source of truth.

No backend may inspect the original specification.

---

## INV-005

Compiler passes transform IR.

Compiler passes never emit Rust directly.

```
IR₁

↓

IR₂

↓

IR₃

↓

Rust
```

---

## INV-006

Inheritance exists only in the specification.

After semantic analysis, inheritance no longer exists.

Generated Rust contains no inheritance hierarchy.

---

## INV-007

Every type reference is resolved before code generation.

The backend never performs name resolution.

---

## INV-008

Choice elements are normalised.

FHIR

```
value[x]
```

must never appear in the final IR.

It becomes

```
ChoiceType
```

---

## INV-009

Cardinality is semantic.

The IR stores cardinality as structured data.

Backends decide how that maps into language constructs.

---

## INV-010

Primitive types are semantic.

The IR distinguishes

* string
* id
* uri
* canonical
* markdown
* instant

even if their wire representation is identical.

---

## INV-011

Documentation is preserved.

Specification documentation is never discarded.

Generated Rustdoc originates from the IR.

---

## INV-012

Terminology bindings are preserved.

Binding strength is retained.

Required

Preferred

Extensible

Example

must remain distinguishable.

---

## INV-013

Constraints are preserved.

Compiler passes may augment constraints.

They must never silently remove them.

---

## INV-014

Generated code is reproducible.

Generated files contain no timestamps.

Generated files contain no machine-specific information.

---

## INV-015

Generated code is formatted.

Every generated crate passes

```
cargo fmt
cargo clippy
```

without warnings.

---

## INV-016

Generated code contains no handwritten logic.

Generation is structural only.

Business logic belongs elsewhere.

---

## INV-017

Backends are independent.

Adding another backend must not require changing the parser.

---

## INV-018

The compiler never edits handwritten code.

Generated output is written only to generated locations.

---

## INV-019

The runtime never depends upon compiler internals.

Compiler crates are build-time tools.

Runtime crates remain usable independently.

---

## INV-020

Every compiler pass is testable.

Each pass has deterministic inputs and outputs suitable for golden testing.

---

# Backend Invariants

The Rust backend additionally guarantees:

* idiomatic module layout
* deterministic ordering
* stable naming
* valid formatting
* compilable output

---

# Runtime Invariants

Generated resources are plain data.

Validation is explicit.

Behaviour is provided by traits.

Builders are generated or derived.

No generated type contains business logic.

---

# Long-Term Invariants

Future compiler features must preserve these principles.

Additional functionality should be implemented by adding compiler passes rather than increasing the complexity of existing passes.

Complexity should move forward through the pipeline, never backward.
