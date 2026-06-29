# AGENTS.md

This repository is designed for both human contributors and AI coding agents.

The rules below are mandatory.

---

# Before Making Changes

Read the following documents:

1. ARCHITECTURE.md
2. STYLE.md
3. PATTERNS.md
4. INVARIANTS.md
5. ROADMAP.md

If a requested change conflicts with these documents, stop and explain the conflict.

---

# Rule 1

Never edit generated code.

Generated files are overwritten by the compiler.

Modify the compiler instead.

---

# Rule 2

Never bypass the Intermediate Representation.

All compiler passes consume IR and produce IR.

No parser may emit Rust.

No backend may inspect XML.

---

# Rule 3

One responsibility per compiler pass.

Compiler passes should be small.

Avoid combining semantic transformations.

---

# Rule 4

Generated code contains structure only.

Do not generate:

- helper methods
- validation
- networking
- persistence

---

# Rule 5

Prefer semantic types.

Do not introduce String when:

- Id
- Uri
- Canonical
- Code

already exist.

---

# Rule 6

Traits describe behaviour.

Do not introduce traits solely to reproduce FHIR inheritance.

---

# Rule 7

Compiler output must be deterministic.

Never introduce:

- timestamps
- random identifiers
- unstable ordering

---

# Rule 8

All public APIs require tests.

Compiler passes require golden tests.

Bug fixes require regression tests.

---

# Rule 9

Follow the canonical patterns in PATTERNS.md.

Do not invent new implementation styles unless they are approved through an ADR.

---

# Rule 10

When in doubt:

- preserve architecture
- minimise complexity
- favour explicit code
- prefer composition