# IR.md

> Specification of the Intermediate Representation (IR).
>
> The IR is the canonical semantic model of FHIR used throughout the compiler.
>
> Every compiler pass consumes IR and produces IR.
>
> The Rust backend consumes only the final IR.

---

# Purpose

The IR exists to separate:

* parsing
* semantic analysis
* code generation

The parser understands FHIR.

The backend understands Rust.

The IR understands neither.

---

# Design Goals

The IR should be:

* language independent
* immutable where practical
* deterministic
* serialisable
* easily testable

It is not intended to model Rust.

It is not intended to model XML.

It models FHIR semantics.

---

# Compiler Pipeline

```
FHIR XML

↓

Specification

↓

Initial IR

↓

Reference Resolution

↓

Flattening

↓

Choice Resolution

↓

Constraint Resolution

↓

Final IR

↓

Rust Backend
```

---

# Root Object

```
Specification
```

Contains:

* resources
* datatypes
* primitives
* value sets
* code systems
* search parameters

---

# Resource

Represents a FHIR resource.

Fields:

```
name

documentation

fields

constraints

bindings
```

Example

```
Patient

Observation

Encounter
```

Resources contain no inheritance.

---

# Datatype

Represents reusable FHIR structures.

Examples

```
HumanName

Identifier

Address

Meta

Coding
```

---

# Primitive

Represents semantic primitive types.

Examples

```
Boolean

Integer

String

Code

Uri

Canonical

Instant

Date

DateTime

Markdown
```

Primitive identity is preserved.

---

# Field

Represents one resource member.

Contains

```
name

type

cardinality

documentation

summary

modifier

must_support

constraints
```

Fields never reference XML.

---

# Type

One of

```
Primitive

Datatype

Resource

Choice

Reference
```

---

# Reference

Represents a reference to another resource.

```
Reference

↓

Patient

Practitioner

Organization
```

Target restrictions remain available.

---

# ChoiceType

Represents FHIR choice elements.

FHIR

```
value[x]
```

IR

```
ChoiceType

variants

base_name
```

The original "[x]" syntax is discarded.

---

# Cardinality

Represents occurrence constraints.

```
Optional

Required

Repeated

RequiredRepeated
```

The Rust backend decides how these map into Rust types.

---

# Constraint

Represents one validation rule.

Contains

```
id

severity

expression

human_description
```

Expression language is preserved.

---

# Binding

Represents terminology bindings.

Contains

```
strength

value_set

documentation
```

Binding strength remains semantic.

---

# Documentation

Documentation is first-class.

The IR stores:

* definition
* comments
* requirements
* examples
* short description

Documentation survives every compiler pass.

---

# Semantic Passes

Each compiler pass has one responsibility.

## Parse

FHIR

↓

Initial IR

---

## Resolve References

Names become references.

---

## Flatten Inheritance

Inherited fields become explicit.

Inheritance disappears.

---

## Resolve Choices

Choice elements become

```
ChoiceType
```

---

## Resolve Cardinality

Cardinality becomes semantic.

---

## Resolve Terminology

Bindings become explicit.

---

## Resolve Constraints

Constraints attach directly to resources and fields.

---

# Backend Contract

The Rust backend assumes:

* inheritance already flattened
* references resolved
* documentation attached
* cardinality resolved
* choice types resolved
* terminology resolved

The backend performs no semantic analysis.

It only generates Rust.

---

# Future Extensions

The IR is expected to grow.

Potential additions include:

* profile constraints
* search definitions
* capability statements
* terminology expansion
* FHIRPath AST
* extension definitions

New compiler passes should extend the IR rather than bypass it.

---

# Non-Goals

The IR does not model:

* Rust syntax
* XML syntax
* JSON syntax
* Serde
* Axum
* SQL
* database persistence

Those belong to downstream backends and runtime libraries.

The IR represents the semantics of FHIR and nothing more.
