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