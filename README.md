# Bridge Crypto

## Inertial Integrity Constraint (IIC)
### A Conformance Framework for Non-Obligatory Execution in Irreversible Systems

Modern crypto systems are extraordinarily good at executing logic.  
They are far less disciplined about deciding **whether execution should occur at all**.

This repository defines, specifies, and *tests for* a foundational governance invariant called the **Inertial Integrity Constraint (IIC)**.

> **IIC is not a mechanism.**  
> **It is not middleware.**  
> **It is not a protocol pattern.**  
>
> **IIC is a property a system either has or lacks.**

This repository exists to make that property **explicit, falsifiable, and testable**.

---

## The Invariant

Most crypto systems operate under an implicit assumption:

> **If an action is validly invoked, execution should follow.**

That assumption optimized early systems for liveness and automation.  
At scale—under adversarial pressure, high value, and irreversible outcomes—it becomes a structural vulnerability.

The **Inertial Integrity Constraint (IIC)** names a different rule of reality:

> **Invocation does not imply obligation to execute.**  
> **Non-execution must be a valid, terminal outcome.**

A system with IIC:
- does **not** treat momentum as permission,
- does **not** assume execution from invocation,
- and does **not** treat "doing nothing" as an error state by default.

---

## What This Repository Is (and Is Not)

This repository is **not** a smart-contract framework.  
It does **not** provide enforcement logic or a canonical implementation of IIC.

Instead, it provides three things:

### 1. A Normative Specification
Defined in `crates/iic-spec`, the spec formalizes IIC as a set of invariant clauses:

- **IIC-1 — Non-Obligation**  
  Invocation alone MUST NOT be sufficient evidence that execution will occur.

- **IIC-2 — Valid Non-Execution**  
  Non-execution MUST be representable as a valid, terminal system outcome.

- **IIC-3 — Explicit Attestation**  
  Only an explicit, checkable attestation may upgrade "invoked" to "will execute."

These clauses define **what it means** for a system to *have* IIC—without prescribing *how* it must be implemented.

---

### 2. A Conformance Harness
Defined in `crates/iic-harness`, the harness provides **property tests** that ask a single question:

> **Does this system exhibit the IIC invariant under examination?**

The harness:
- does **not** enforce behavior,
- does **not** dictate architecture,
- and does **not** encode policy.

It exists solely to make IIC **falsifiable**.

A system may pass or fail the harness.  
That result says nothing about *how* the system is built—only whether it exhibits the property.

---

### 3. A Witness Implementation (Non-Normative)
Defined in `crates/iic-witness`, the witness is a **minimal, illustrative example** whose only purpose is to:

- demonstrate how the harness can be exercised,
- provide a concrete target for tests,
- and anchor abstract concepts in something executable.

> **The witness is not "IIC."**  
> It is one possible expression among many.

It exists to prevent ambiguity—not to define the invariant.

---

## Why This Structure Matters

IIC is intentionally framed as an **invariant**, not a pattern.

If IIC were delivered as:
- a single contract,
- a reusable library,
- or a mandatory execution gate,

it would collapse from a property of systems into a feature of code.

This repository avoids that failure mode by separating:
- **specification** (what must be true),
- **conformance** (how we test for truth),
- and **witnesses** (examples, not definitions).

The result is a framework that can be adopted, challenged, and extended—without being reduced.

---

## Repository Structure

```
bridge-crypto/
├── Cargo.toml                    # Workspace root
├── README.md                     # This file
└── crates/
    ├── iic-spec/
    │   └── README.md             # Normative clauses (IIC-1/2/3)
    ├── iic-core/
    │   ├── Cargo.toml
    │   └── src/lib.rs            # Traits + types (witness surface)
    ├── iic-harness/
    │   ├── Cargo.toml
    │   ├── src/lib.rs            # Conformance checks
    │   └── tests/conformance.rs  # Property tests
    └── iic-witness/
        ├── Cargo.toml
        └── src/lib.rs            # Minimal witness implementation
```

---

## Quick Start

```bash
# Clone the repo
git clone https://github.com/Griffinwalters-Bridgetech/bridge-crypto.git
cd bridge-crypto

# Run the conformance tests
cargo test
```

If the tests pass, the witness implementation exhibits IIC.

To test your own system, implement the traits in `iic-core` and run the harness against your implementation.

---

## Portfolio Context: The Bridge Architecture

`bridge-crypto` is one expression of a broader governance thesis shared across the Bridge portfolio.

| Project | Domain | Core Function |
|---------|--------|---------------|
| **Bridge OS** | AI Governance | Enforces the `.03` Principle at the schema level — systems cannot complete autonomously |
| **Bridge Crypto (IIC)** | Crypto Governance | Tests whether systems exhibit non-obligatory execution |
| **Eyes Unseen** | Consumer Attention | Measures manipulation — engagement ≠ consent |
| **DataPacks** | Knowledge Systems | Preserve expert judgment — extraction ≠ understanding |

### The Shared Invariant

Across domains, the same principle appears in different forms:

| Expression | Domain | Invariant |
|------------|--------|-----------|
| `.03 Principle` | AI | Completion ≠ entitlement |
| `IIC` | Crypto | Invocation ≠ obligation |
| Attention Measurement | Consumer | Engagement ≠ consent |
| DataPacks | Knowledge | Output ≠ understanding |

These are not separate products.  
They are **one governance architecture expressed in four domains**.

If the same invariant holds across AI, crypto, attention, and knowledge systems, that is not a feature set.

It is a claim about **how systems should relate to power, momentum, and human agency**.

---

## How to Use This Repository

- **Protocol designers:** test whether your system assumes execution by default.
- **Auditors:** identify whether non-execution is treated as a valid terminal state.
- **Researchers:** explore invariant-level governance without prescribing implementation.
- **Critics:** try to break the invariant—this repo is designed to allow that.

---

## Status

This repository is intentionally early-stage and intentionally opinionated at the invariant level.

Expect:
- refinement of the spec language,
- expansion of the harness,
- additional witnesses in distinct contexts.

Do **not** expect:
- a "drop-in IIC contract,"
- a universal execution gate,
- or enforcement logic disguised as philosophy.

---

## Related Repositories

- [Bridge OS](https://github.com/Griffinwalters-Bridgetech/Bridge-os-governance) — AI governance API

---

## Contact

**Bridge Technologies**  
**99.97 Labs**

---

## Final Note

IIC does not stop systems from acting.  
It stops systems from *assuming they are entitled to act*.

That distinction is the work.
