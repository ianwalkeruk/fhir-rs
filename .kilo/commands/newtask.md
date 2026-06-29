---
description: Plan, implement and complete the next roadmap task.
agent: code
model: kilo/kilo-auto/free
---

# Automated Task Management

Execute the following steps sequentially.

You are an orchestration agent.

Your responsibilities are limited to:

- coordinating subagents
- managing branches
- committing work
- updating project status
- creating the pull request

You MUST NOT analyse project architecture or source code yourself.
Delegate all implementation work to specialist agents.

---

## 1. Generate TASK.md

Launch a subagent with the following prompt.

```
Read the following project documents:

- ROADMAP.md
- ARCHITECTURE.md
- CURRENT.md (if present)
- STYLE.md
- PATTERNS.md
- INVARIANTS.md
- IR.md (if compiler work)

Identify the highest-priority roadmap item that:

- is not complete
- fits within a single implementation task
- remains entirely within one roadmap epic
- does not violate any architectural invariant

Generate TASK.md using TASK.template.md.

TASK.md should reference only the documentation required for this task.

Do not perform any implementation.
```

---

## 2. Create feature branch

Create a new local feature branch.

Use the task title from TASK.md as the basis of the branch name.

Do not push.

---

## 3. TDD Phase

Launch a subagent.

```
Read TASK.md.

This is an interface-first task.

Implement only:

- public interfaces
- traits
- type definitions
- failing tests

Do NOT implement functionality.

Follow every referenced project document.

Use:

- unit tests
- property tests (proptest)
- snapshot / golden tests where appropriate
- integration tests where appropriate

Stop once the project compiles with intentionally failing tests.

Provide a concise summary of the interfaces created.
```

---

## 4. Commit TDD work

Commit the interface and test changes.

Do not push.

---

## 5. Implementation Phase

Launch another subagent.

```
Read TASK.md.

Implement the interfaces created during the previous task.

Do not extend the scope.

If additional work is discovered:

- do not implement it
- add it to ROADMAP.md

Ensure all tests pass.

Run:

cargo fmt --all

cargo clippy --workspace --all-targets --all-features -- -D warnings

cargo test --workspace

Provide a concise summary of the completed implementation.
```

---

## 6. Documentation

Update:

- HISTORY.md
- ROADMAP.md
- CURRENT.md (if present)

Only update:

- ARCHITECTURE.md
- STYLE.md
- PATTERNS.md
- INVARIANTS.md
- IR.md
- ADRs

when required by architectural changes.

Ensure Dockerfile copies any newly-created crates.

---

## 7. Cleanup

Delete TASK.md.

TASK.md is ephemeral and MUST NOT remain in the repository.

---

## 8. Commit

Create a detailed commit.

---

## 9. Pull Request

Create a pull request from the feature branch into main.

Do nothing else.