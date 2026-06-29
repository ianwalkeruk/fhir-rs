# STYLE.md

> Coding standards for the FHIR Rust project.
>
> This document is intentionally prescriptive. Consistency is preferred over personal preference.

---

# Philosophy

Code should be:

* Idiomatic Rust
* Easy to read
* Easy to generate
* Easy to review
* Easy to refactor

Prefer obvious solutions over clever ones.

Generated code should be mechanically simple.
Handwritten code should provide the project's ergonomics.

---

# General Principles

## Prefer composition over inheritance

FHIR models inheritance.

Rust models composition.

Inheritance is flattened during compilation.

✓

```rust
pub struct Patient {
    pub id: Option<Id>,
    pub meta: Option<Meta>,
    pub identifier: Vec<Identifier>,
}
```

✗

```rust
pub struct Patient {
    base: DomainResource,
}
```

---

## Traits describe behaviour

Traits represent capabilities.

✓

```rust
trait Validate {}

trait Resource {}

trait HasId {}
```

✗

```rust
trait DomainResource {}

trait BackboneElement {}
```

---

## Prefer enums over trait objects

✓

```rust
enum Resource {
    Patient(Patient),
    Observation(Observation),
}
```

✗

```rust
Box<dyn Resource>
```

Trait objects require justification.

---

## Prefer newtypes over aliases

✓

```rust
pub struct Id(String);

pub struct Uri(String);
```

✗

```rust
type Id = String;
```

Newtypes provide type safety.

---

## Avoid primitive obsession

Do not expose `String` where a semantic type exists.

Prefer:

* Id
* Uri
* Canonical
* Code
* Instant
* Date
* Markdown

---

## Strong typing beats convenience

Prefer

```rust
Reference<Patient>
```

over

```rust
Reference
```

where practical.

---

# Generated Code

Generated code should contain:

* structs
* enums
* derives
* documentation
* constants

Generated code should not contain:

* business logic
* validation
* helper methods
* formatting logic
* networking code

---

# Builder Pattern

Resources with many fields should expose builders.

Required fields should use the Type State Builder pattern where practical.

---

# Validation

Validation is explicit.

Avoid validating during construction unless required to preserve type invariants.

Prefer:

```rust
resource.validate()?;
```

over

```rust
Patient::new(...) -> Result<...>
```

---

# Error Handling

Libraries return structured errors.

Never panic because of invalid FHIR input.

Reserve panic for internal compiler bugs.

---

# Ownership

Prefer borrowing.

Clone only when ownership is required.

Avoid unnecessary allocation.

---

# Collections

Prefer

```rust
Vec<T>
```

over linked structures.

Prefer slices for read-only APIs.

---

# Options

Represent FHIR cardinality directly.

| FHIR | Rust                          |
| ---- | ----------------------------- |
| 0..1 | Option<T>                     |
| 1..1 | T                             |
| 0..* | Vec<T>                        |
| 1..* | Non-empty collection (future) |

---

# Documentation

Public APIs require Rustdoc.

Generated documentation should originate from the specification.

Handwritten documentation explains behaviour.

---

# Formatting

Always use:

```
cargo fmt
cargo clippy
```

Warnings are treated as defects.

---

# Dependencies

Minimise dependencies.

Prefer the standard library.

Introduce new dependencies only when they significantly reduce complexity.

---

# Unsafe

Unsafe code is prohibited unless approved through an ADR.

---

# Testing

Every public API requires tests.

Compiler passes require golden tests.

Bug fixes require regression tests.

---

# Naming

Prefer explicit names.

✓

```rust
PatientRepository
ObservationStatus
IdentifierSystem
```

Avoid abbreviations.

---

# Modules

Small modules are preferred.

One responsibility per module.

Avoid "utils.rs".

---

# Comments

Explain *why*.

Avoid comments that restate code.

✓

```rust
// FHIR permits duplicate identifiers with different systems.
```

✗

```rust
// Increment i.
i += 1;
```
