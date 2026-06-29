# ROADMAP.md

This roadmap intentionally favours small, reviewable changes.

Each phase should be independently testable and releasable.

---

# Milestone 0

Repository Foundation

## Epic 0.1

Workspace

Deliverables

* Cargo workspace
* CI
* formatting
* clippy
* deny
* audit

Success Criteria

Repository builds cleanly.

---

## Epic 0.2

Golden testing framework

Deliverables

* snapshot tests
* compiler harness

Success Criteria

Generated code is regression tested.

---

## Epic 0.3 ✓

### Intermediate Representation Definition

**Goal**

Define the project's canonical Intermediate Representation (IR) as a versioned, language-independent schema before implementing any compiler passes.

The IR is considered part of the project's public architecture.

All compiler passes consume and produce this representation.

**Status: COMPLETE**

---

### Objectives

* [x] Define the complete IR schema.
* [x] Make the IR language-independent.
* [x] Ensure the IR is serialisable.
* [x] Version the IR independently of the FHIR specification.
* [x] Support deterministic YAML serialisation.

---

### Deliverables

#### IR Schema ✓

Create the core IR definitions representing:

* [x] Specification
* [x] Resource
* [x] Datatype
* [x] Primitive
* [x] Field
* [x] Type
* [x] ChoiceType
* [x] Reference
* [x] Cardinality
* [x] Constraint
* [x] Binding
* [x] Documentation

---

#### Rust Model ✓

Implement the IR as Rust structures.

Requirements:

* [x] `Serialize`
* [x] `Deserialize`
* [x] `Debug`
* [x] `Clone`
* [x] `PartialEq`

The Rust model is the canonical implementation of the schema.

---

#### YAML Representation ✓

Provide deterministic YAML serialisation.

Example:

```yaml
ir_version: 0.1.0

resources:

  - name: Patient

    fields:

      - name: identifier
        type: Identifier
        cardinality: Repeated

      - name: active
        type: Boolean
        cardinality: Optional
```

YAML output is stable between compiler runs.

---

#### IR Versioning ✓

Introduce an independent IR version.

Example:

```text
IR Version

0.1.0
```

The IR version changes only when the schema changes.

FHIR versioning remains separate.

---

#### Golden Tests ✓

Create snapshot tests verifying:

* [x] deterministic YAML
* [x] schema stability
* [x] serialisation round-tripping

---

#### Documentation ✓

Produce:

* [x] `IR.md`
* [x] schema documentation
* [x] example IR documents

---

### Exit Criteria

The project can:

* [x] construct an IR in memory
* [x] serialise it to YAML
* [x] deserialize it back
* [x] verify equality after round-trip
* [x] produce deterministic snapshot output

No compiler logic is implemented during this epic.

The objective is to establish the compiler's semantic foundation.

---

### Future Work Enabled

Completion of this epic enables:

* parser implementation
* semantic analysis passes
* code generation
* backend experimentation
* compiler visualisation
* IR diff tooling
* pass-by-pass debugging
* alternative language backends

---

## Epic 0.4

### Compiler CLI

**Goal**

Provide a standalone command-line interface for inspecting, validating and manipulating the compiler's Intermediate Representation (IR).

The CLI is a development and debugging tool.

It is not a code generator.

---

### Objectives

* Make compiler state observable.
* Support pass-by-pass debugging.
* Enable deterministic testing.
* Provide tooling for future compiler development.
* Decouple compiler inspection from code generation.

---

### Deliverables

#### CLI Application

Create a standalone executable.

Example:

```text
fhirc
```

The CLI should be lightweight and depend only upon compiler crates.

---

#### Command Structure

Initial commands:

```text
fhirc version
```

Display:

* compiler version
* IR version
* supported FHIR release(s)

---

```text
fhirc schema
```

Display the supported IR schema version.

---

```text
fhirc validate-ir patient.ir.yaml
```

Validate an IR document against the current schema.

Exit with a non-zero status on validation failure.

---

```text
fhirc pretty patient.ir.yaml
```

Pretty-print an IR document in a canonical format.

Useful for reviews and debugging.

---

```text
fhirc canonicalise patient.ir.yaml
```

Normalise ordering and formatting.

Output must be deterministic.

---

#### Future Commands (Stub Only)

Reserve command names for future functionality.

These commands may initially return "not yet implemented".

```text
fhirc parse specification.xml

fhirc pass resolve-references

fhirc pass flatten

fhirc pass resolve-choices

fhirc generate rust

fhirc diff before.ir.yaml after.ir.yaml
```

This establishes the long-term CLI interface without committing to implementation.

---

### Testing

Create integration tests verifying:

* command execution
* deterministic output
* stable YAML formatting
* exit codes
* invalid input handling

CLI output should be suitable for golden tests.

---

### Documentation

Provide:

* command reference
* examples
* expected workflows

Example workflow:

```text
FHIR XML

↓

fhirc parse

↓

initial.ir.yaml

↓

fhirc pretty

↓

fhirc validate-ir

↓

validated.ir.yaml
```

---

### Exit Criteria

The project provides:

* a functioning CLI executable
* deterministic YAML formatting
* IR validation
* schema reporting
* integration tests
* user documentation

No compiler passes are implemented during this epic.

The CLI establishes the project's primary debugging and inspection interface.

---

### Future Work Enabled

Completion of this epic enables:

* compiler pass visualisation
* IR diff tooling
* regression testing
* backend experimentation
* IDE integration
* scripting and automation
* CI validation of IR snapshots

The CLI should become the preferred method for inspecting compiler behaviour throughout the lifetime of the project.

---

# Milestone 1

Compiler Front-End

Goal

Read official FHIR specification artifacts.

## Epic 1.1

Specification loader

* StructureDefinitions
* ValueSets
* CodeSystems

## Epic 1.2

XML parser

Produces internal specification objects.

## Epic 1.3

Reference resolver

Links all referenced types.

## Epic 1.4

Inheritance graph

Produces complete dependency graph.

Exit Criteria

FHIR specification successfully loads.

---

# Milestone 2

Intermediate Representation

Goal

Construct language-neutral semantic model.

## Epic 2.1

Primitive definitions

## Epic 2.2

Complex datatypes

## Epic 2.3

Resources

## Epic 2.4

Choice elements

## Epic 2.5

Cardinality

## Epic 2.6

Documentation

Exit Criteria

IR completely represents Patient.

---

# Milestone 3

Rust Code Generation

Goal

Generate compilable Rust.

## Epic 3.1

Primitive emitter

## Epic 3.2

Datatype emitter

## Epic 3.3

Patient emitter

## Epic 3.4

Terminology enums

## Epic 3.5

Module generation

Exit Criteria

Generated crate compiles without warnings.

---

# Milestone 4

Core Runtime

Goal

Provide handwritten runtime infrastructure.

## Epic 4.1

Primitive wrappers

## Epic 4.2

Traits

## Epic 4.3

Reference<T>

## Epic 4.4

Resource enum

Exit Criteria

Generated Patient compiles against runtime.

---

# Milestone 5

JSON Support

Goal

Round-trip Patient resources.

## Epic 5.1

Serde integration

## Epic 5.2

Primitive extensions

## Epic 5.3

Choice serialization

## Epic 5.4

Official example tests

Exit Criteria

Patient JSON round-trips successfully.

---

# Milestone 6

Builders

Goal

Provide ergonomic resource construction.

## Epic 6.1

Standard builders

## Epic 6.2

Type-state builders

## Epic 6.3

Collection helpers

## Epic 6.4

Builder documentation

Exit Criteria

Required fields enforced where practical at compile time.

---

# Milestone 7

Validation

Goal

Detect structural errors.

## Epic 7.1

Primitive validation

## Epic 7.2

Required elements

## Epic 7.3

Cardinality

## Epic 7.4

Constraint framework

Exit Criteria

Generated Patient validates successfully.

---

# Milestone 8

MVP FHIR Server

Goal

Store and retrieve Patient resources.

## Epic 8.1

Axum server

## Epic 8.2

Repository abstraction

## Epic 8.3

In-memory repository

## Epic 8.4

POST /Patient

## Epic 8.5

GET /Patient/{id}

## Epic 8.6

PUT /Patient/{id}

## Epic 8.7

DELETE /Patient/{id}

## Epic 8.8

CapabilityStatement

Exit Criteria

FHIR client can create and retrieve Patient resources.

---

# Milestone 9

Persistence

## Epic 9.1

SQLite repository

## Epic 9.2

Transactions

## Epic 9.3

Migration framework

Exit Criteria

Server survives restart.

---

# Milestone 10

Search

## Epic 10.1

family

## Epic 10.2

given

## Epic 10.3

identifier

## Epic 10.4

birthDate

Exit Criteria

Basic FHIR search supported.

---

# Milestone 11

Additional Resources

* Practitioner
* Organization
* Encounter
* Observation

Compiler infrastructure should require minimal modification.

---

# Milestone 12

XML

Implement full FHIR XML support.

---

# Milestone 13

Profiles

Generate profile-specific Rust types.

Examples:

* UK Core
* US Core
* IPS

---

# Milestone 14

FHIRPath

Compile FHIRPath constraints into Rust.

Long-term objective:

Move as much validation as possible from runtime to compile time.