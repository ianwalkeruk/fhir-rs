# fhir-rs

> An idiomatic Rust implementation of HL7 FHIR, built by compiling the FHIR specification into Rust.

## Vision

FHIR is traditionally implemented using object-oriented models, reflecting its Java origins.

This project takes a different approach.

Rather than translating the FHIR object model into Rust, it compiles the FHIR specification into a strongly typed, idiomatic Rust API that embraces:

- Composition over inheritance
- Traits describing behaviour
- Strong semantic types
- Zero-cost abstractions
- Compile-time correctness where practical

## Project Goals

- Generate Rust models directly from the official FHIR specification.
- Preserve FHIR semantics through a language-independent Intermediate Representation.
- Provide ergonomic builders and validation.
- Support JSON and XML serialisation.
- Provide a reference FHIR server.
- Make invalid FHIR harder to construct than valid FHIR.

## Repository Layout

compiler/     Build-time compiler
generated/    Generated model crate
crates/       Handwritten runtime libraries
macros/       Procedural macros

## Documentation

Start here:

- ARCHITECTURE.md
- ROADMAP.md
- STYLE.md
- PATTERNS.md
- IR.md
- INVARIANTS.md

## Status

This project is under active development.

APIs and crate structure should be considered unstable until the first release.

## License

Apache-2.0 OR MIT