# HISTORY.md

## 2026-06-30

### XML Parser Interfaces (Task 1.2)

Implemented the XML parser public interfaces and failing tests for the FHIR compiler's Source Layer.

**Changes:**

- Created `compiler/fhir-parser/src/parse.rs` with `Parse` trait producing `fhir-ir::Specification`
- Created `compiler/fhir-parser/src/parse_error.rs` with `ParseError` enum covering structured XML failure modes
- Created `compiler/fhir-parser/src/xml_types.rs` with XML token stream, event, name, attribute, and source types
- Added FHIR XML artifact types: `StructureDefinitionXml`, `ValueSetXml`, `CodeSystemXml` with supporting enums
- Added failing tests:
  - `tests/parse_trait.rs` - Parse trait contract tests
  - `tests/parse_error.rs` - ParseError variant coverage tests
  - `tests/xml_types.rs` - XML type construction and accessor tests
  - `tests/proptest.rs` - Property tests for determinism (5 tests)
  - `tests/snapshot_tests.rs` - Snapshot/determinism tests (3 tests)
- Updated `fhirc` CLI with `parse` command stub returning `NotImplemented`

**Tests:** 44 parser tests passing, including trait existence, error variants, and determinism assertions.

## 2026-06-30

### Specification Loader Interfaces (Task 1.1)

Implemented the specification loader interfaces for loading FHIR specification artifacts.

**Changes:**

- Created `compiler/fhir-parser/src/error.rs` with `LoaderError` enum and `LoaderResult` type
- Created `compiler/fhir-parser/src/loader.rs` with `Loader` trait for loading complete specifications
- Created `compiler/fhir-parser/src/loader_resource.rs` with `ResourceLoader` trait for StructureDefinition/Datatype loading
- Created `compiler/fhir-parser/src/loader_valueset.rs` with `ValueSetLoader` trait for ValueSet loading
- Created `compiler/fhir-parser/src/loader_codesystem.rs` with `CodeSystemLoader` trait for CodeSystem loading
- Added integration tests in `compiler/fhir-parser/tests/loader_traits.rs` verifying trait definitions and error variants

**Tests:** 6 tests passing, including trait existence tests and error variant coverage.

## 2026-06-29

### Golden Testing Framework (Task 0.2)

Implemented the GoldenTest trait for all IR types and added comprehensive property-based testing infrastructure.

**Changes:**

- Created `compiler/fhir-ir/src/golden_test.rs` with GoldenTest trait and utility functions
- Implemented GoldenTest for all IR types (Resource, Datatype, Field, Type, Cardinality, ChoiceType, Reference, Primitive, Constraint, Binding, Documentation, Specification, etc.)
- Added `compiler/fhir-ir/tests/golden_test.rs` with property tests for serialization invariants
- Added determinant, round-trip, and timestamp-free assertions

**Tests:** 94 tests passing, including 8 new golden tests and 4 property tests verifying INV-001 deterministic output.

## 2026-06-29

### Workspace Foundation (Task 0.1)

Implemented workspace foundation infrastructure including CI, formatting, and security tools.

**Changes:**

- Created `.github/workflows/ci.yml` with jobs for rustfmt, clippy, build, test, and security
- Created `.rustfmt.toml` with stable-compatible formatting rules
- Created `clippy.toml` with workspace-wide lint thresholds
- Created `deny.toml` for cargo-deny license and vulnerability checking

**Tests:** All 69 workspace tests passing.

## 2026-06-29

### IR Schema Rust Structures (Task 0.3.1)

Implemented the Intermediate Representation schema as idiomatic Rust structures with Serialize, Deserialize, Debug, Clone, and PartialEq traits.

**Changes:**

- Created core IR types in `compiler/fhir-ir/src/`:
  - `cardinality.rs` - Cardinality enum (Optional, Required, Repeated, RequiredRepeated)
  - `reference.rs` - Reference struct with target restrictions
  - `choice_type.rs` - ChoiceType for FHIR choice elements
  - `documentation.rs` - Documentation struct preserving spec docs
  - `constraint.rs` - Constraint struct for validation rules
  - `binding.rs` - Binding struct for terminology bindings
  - `field.rs` - Field struct for resource/datatype members
  - `primitive.rs` - Primitive struct for semantic primitive types
  - `datatype.rs` - Datatype struct for complex types
  - `resource.rs` - Resource struct for FHIR resources
  - `specification.rs` - Root Specification struct
  - `type.rs` - Type enum for all type references

- Added YAML serialization tests in `tests/snapshot_tests.rs`
- Added golden test snapshot at `tests/patient.ir.yaml`

**Tests:** 47 tests passing, including property tests and snapshot tests for deterministic serialization.

## 2026-06-29

### Compiler CLI (Task 0.4)

Implemented the `fhirc` command-line interface for inspecting, validating, and manipulating the Intermediate Representation.

**Changes:**

- Created CLI executable in `compiler/fhirc/`:
  - `cli.rs` - Clap-based command parser with `Commands` enum and semantic types (`IrPath`, `Output`)
  - `commands.rs` - `CliCommands` implementing `Command` trait with all subcommands
  - `error.rs` - Semantic error types using `thiserror`
  - `exit_code.rs` - Exit code constants and error mapping

- Implemented commands:
  - `fhirc version` - Displays compiler version, IR version, and supported FHIR releases
  - `fhirc schema` - Displays supported IR schema version
  - `fhirc validate-ir <file>` - Validates an IR document and exits non-zero on failure
  - `fhirc pretty <file>` - Pretty-prints an IR document in canonical format
  - `fhirc canonicalise <file>` - Normalises ordering and formatting deterministically
  - `fhirc parse` - Reserved future command (not yet implemented)
  - `fhirc pass <name>` - Reserved future command (not yet implemented)
  - `fhirc generate` - Reserved future command (not yet implemented)
  - `fhirc diff` - Reserved future command (not yet implemented)

- Added integration tests in `compiler/fhirc/tests/cli_tests.rs` verifying command execution, deterministic output, and exit codes
- Added property tests in `compiler/fhirc/tests/proptest.rs` for deterministic invariants
- Added golden test snapshots for version and schema output

**Tests:** 69 tests passing, including unit, integration, property, and snapshot tests.