# HISTORY.md

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