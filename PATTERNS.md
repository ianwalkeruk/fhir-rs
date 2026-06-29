# PATTERNS.md

> Canonical implementation patterns used throughout the project.
>
> New code should follow these examples unless an Architectural Decision Record states otherwise.

---

# Resource Structure

```rust
pub struct Patient {
    pub id: Option<Id>,
    pub meta: Option<Meta>,
    pub identifier: Vec<Identifier>,
    pub active: Option<bool>,
}
```

Resources are plain data.

---

# Behaviour via Traits

```rust
impl Resource for Patient {
    fn resource_type(&self) -> ResourceType {
        ResourceType::Patient
    }
}
```

---

# Resource Enumeration

```rust
pub enum Resource {
    Patient(Patient),
    Observation(Observation),
}
```

Enums replace inheritance.

---

# Typed References

```rust
pub struct Reference<T> {
    pub reference: Id,
    marker: PhantomData<T>,
}
```

Avoid untyped references where possible.

---

# Builder

```rust
let patient = Patient::builder()
    .identifier(identifier)
    .active(true)
    .build();
```

Builders improve readability for large resources.

---

# Type State Builder

```rust
PatientBuilder<MissingIdentifier>

↓

PatientBuilder<HasIdentifier>
```

Only valid states expose `build()`.

---

# Choice Elements

FHIR

```
value[x]
```

Rust

```rust
pub enum ObservationValue {
    Quantity(Quantity),
    String(String),
    Boolean(bool),
}
```

---

# Cardinality Mapping

FHIR

```
0..1
```

↓

```rust
Option<T>
```

FHIR

```
1..1
```

↓

```rust
T
```

FHIR

```
0..*
```

↓

```rust
Vec<T>
```

---

# Repository Pattern

```rust
pub trait PatientRepository {
    async fn get(&self, id: &Id) -> Result<Option<Patient>>;

    async fn save(&self, patient: Patient) -> Result<()>;
}
```

Infrastructure depends on traits.

Storage implementations remain interchangeable.

---

# Validation

```rust
patient.validate()?;
```

Validation is separate from construction.

---

# Compiler Pipeline

```
FHIR XML

↓

Specification

↓

Intermediate Representation

↓

Semantic Analysis

↓

Rust Generation
```

Each stage has a single responsibility.

---

# Code Generation

```
FHIR

↓

IR

↓

Generated Rust

↓

cargo fmt

↓

Golden Tests
```

Never emit directly from XML.

---

# Extension Traits

Generated code remains simple.

Additional behaviour belongs in extension traits.

```rust
pub trait PatientExt {
    fn official_name(&self) -> Option<&HumanName>;
}
```

---

# Error Types

Prefer structured errors.

```rust
pub enum ValidationError {
    MissingField(String),
    InvalidCardinality(String),
}
```

Avoid stringly-typed errors.

---

# Module Layout

```
patient.rs

patient_builder.rs

patient_validation.rs
```

Prefer many focused modules over large files.

---

# Golden Tests

Compiler output is tested using snapshot files.

```
FHIR Definition

↓

Generated Rust

↓

Snapshot Comparison
```

---

# Handwritten vs Generated

Generated:

* data structures
* derives
* documentation

Handwritten:

* builders
* validation
* repositories
* HTTP
* persistence
* helper methods

Keep this boundary strict.
