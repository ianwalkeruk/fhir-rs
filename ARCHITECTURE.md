# Architecture

> **FHIR is compiled, not translated.**
>
> This project does not attempt to reproduce the Java implementation of FHIR in Rust.
> Instead, it treats the FHIR specification as a schema language and compiles that schema into idiomatic Rust.

---

# Vision

The goal of this project is to produce a high-quality, idiomatic Rust implementation of the HL7 FHIR standard.

Rather than modelling Java's object-oriented hierarchy, the project embraces Rust's strengths:

* algebraic data types
* composition
* traits describing behaviour
* strong static typing
* compile-time correctness
* code generation
* zero-cost abstractions

Where the FHIR specification describes *what* a resource is, this compiler determines *how* that resource should be represented in Rust.

---

# Core Philosophy

The project follows a simple principle:

> Parse. Transform. Emit.

Each stage has exactly one responsibility.

```text
FHIR Specification
        │
        ▼
     Parse
        │
        ▼
Intermediate Representation
        │
Semantic Compiler Passes
        │
        ▼
   Canonical IR
        │
        ▼
      Emitters
        │
        ├── Rust
        ├── Documentation
        ├── GraphViz
        └── Future Backends
```

No stage bypasses another.

---

# Architectural Principles

## The compiler owns semantics

Semantic analysis belongs exclusively within compiler passes.

Examples include:

* inheritance
* references
* cardinality
* terminology bindings
* choice elements
* constraints

Backends never perform semantic analysis.

---

## The Intermediate Representation is canonical

The Intermediate Representation (IR) is the single source of truth.

Every compiler pass consumes IR and produces IR.

Every backend consumes only the final IR.

Neither parsers nor emitters communicate directly.

---

## Rust is one backend

Rust is not the purpose of the compiler.

Rust is one possible backend.

Future backends may generate:

* documentation
* GraphViz diagrams
* JSON Schema
* OpenAPI
* TypeScript

The compiler architecture should not change when adding new emitters.

---

## Generated code is structural

Generated Rust should contain only information present within the specification.

Generated code may contain:

* structs
* enums
* derives
* documentation
* constants

Generated code should not contain:

* business logic
* helper methods
* networking
* persistence
* validation logic

---

## Handwritten code provides ergonomics

Everything that improves usability belongs outside generated code.

Examples include:

* builders
* validation
* extension traits
* repositories
* HTTP servers
* procedural macros

---

## Rust should remain recognisably Rust

The generated API should feel like an idiomatic Rust library.

It should not resemble a Java object model translated into Rust syntax.

Examples:

* composition instead of inheritance
* enums instead of runtime polymorphism
* traits describing behaviour
* strong semantic types
* explicit ownership

---

# Compiler Architecture

The compiler consists of three layers.

## Source Layer

Responsible for understanding external formats.

Examples:

* StructureDefinitions
* XML
* JSON
* Profiles
* ValueSets
* CodeSystems

Output:

Initial IR

Source parsers perform no semantic analysis.

---

## Semantic Layer

Responsible for understanding FHIR.

Compiler passes transform the IR into progressively richer semantic forms.

Typical passes include:

* reference resolution
* inheritance flattening
* choice resolution
* cardinality resolution
* terminology resolution
* constraint attachment

Each pass performs one transformation.

Compiler passes should remain deterministic and independently testable.

---

## Backend Layer

Responsible for rendering artefacts.

Backends consume the final IR.

Examples include:

* Rust source
* documentation
* dependency graphs

Backends never infer missing semantic information.

---

# Compiler Pipeline

The compiler is designed as a sequence of small transformations.

```text
Specification

↓

Initial IR

↓

Resolve References

↓

Flatten Inheritance

↓

Resolve Choice Types

↓

Resolve Cardinality

↓

Resolve Terminology

↓

Resolve Constraints

↓

Final IR

↓

Emit Rust
```

Each transformation should be executable independently.

---

# Command-Line Interface

The compiler is exposed through a composable command-line interface.

Commands belong to one of three categories.

## Parse

External formats → IR

Examples:

* parse XML
* parse profiles

---

## Pass

IR → IR

Examples:

* resolve-references
* flatten
* resolve-choices

Compiler passes should compose naturally through Unix pipelines.

---

## Emit

IR → Output

Examples:

* emit Rust
* emit documentation
* emit GraphViz

---

# Workspace Organisation

The repository is organised around architectural responsibilities rather than implementation stages.

```text
compiler/

    parser/

    ir/

    passes/

    emit/

    cli/

runtime/

    core/

    model/

    json/

    validation/

    server/

macros/
```

Compiler crates should remain independent of runtime crates.

Runtime crates should never depend upon compiler internals.

---

# Runtime Architecture

The runtime consists of handwritten libraries built on top of generated models.

Responsibilities include:

* semantic primitive types
* builders
* validation
* repositories
* HTTP services
* persistence

Generated models remain plain data structures.

Behaviour is introduced through traits and helper libraries.

---

# Architectural Decision Records

The following principles are considered stable.

## ADR-001

FHIR is compiled rather than translated.

Status: Accepted

---

## ADR-002

The Intermediate Representation is the compiler's canonical data model.

Status: Accepted

---

## ADR-003

Compiler passes transform IR into IR.

Status: Accepted

---

## ADR-004

Semantic analysis is complete before emission begins.

Status: Accepted

---

## ADR-005

Inheritance is flattened during compilation.

Status: Accepted

---

## ADR-006

Generated code contains structure only.

Status: Accepted

---

## ADR-007

Traits describe behaviour rather than taxonomy.

Status: Accepted

---

## ADR-008

Validation is explicit.

Status: Accepted

---

## ADR-009

The compiler follows a Parse → Transform → Emit architecture.

Status: Accepted

---

## ADR-010

The command-line interface is pipeline-oriented.

Status: Accepted

---

# Long-Term Direction

The architecture is intended to support future capabilities without requiring structural redesign.

Examples include:

* additional FHIR releases
* profile-aware code generation
* compiled FHIRPath
* terminology expansion
* additional language backends
* interactive compiler tooling

These features should emerge by adding new compiler passes and emitters rather than changing the fundamental architecture.

The architecture should remain stable even as the implementation evolves.
