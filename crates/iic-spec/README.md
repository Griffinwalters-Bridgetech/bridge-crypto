# IIC Specification

## Inertial Integrity Constraint — Normative Clauses

This document defines the IIC invariant as a safety property. A system either exhibits IIC or it does not. These clauses define what counts as violation.

---

## IIC-1: Non-Obligation

> **An invocation, event, or request MUST NOT be treated as sufficient evidence that an action will execute.**

Invocation is not obligation. The act of calling, proposing, or requesting does not by itself create a commitment to proceed.

**Violation:** A system that automatically executes upon invocation without an explicit attestation step violates IIC-1.

---

## IIC-2: Non-Execution Validity

> **A terminal non-execution outcome MUST be representable and MUST NOT be considered an error state by default.**

"Did not execute" is a valid outcome, not a failure. Systems must have a way to represent this as a complete, correct terminal state.

**Violation:** A system that treats non-execution as timeout, error, or exception violates IIC-2.

---

## IIC-3: Explicit Attestation

> **Only an explicit attestation artifact may upgrade "invoked" → "will execute" (and the artifact must be checkable).**

The transition from "requested" to "committed to execute" requires an explicit, verifiable step. This attestation is the boundary between invocation and obligation.

**Violation:** A system that implicitly assumes execution after invocation without a checkable attestation step violates IIC-3.

---

## Relationship to Bridge OS (.03 Principle)

IIC is the crypto-native expression of the same invariant that Bridge OS enforces for AI systems:

| Domain | Expression |
|--------|------------|
| AI (Bridge OS) | System cannot complete 100% autonomously. Human gate required. |
| Crypto (IIC) | Invocation does not imply obligation to execute. Non-execution is valid. |

The underlying principle: **momentum is not permission.**

---

## How to Test for IIC

Use the conformance harness in `iic-harness`. Bring your own system. Run the tests. See if it passes.

The harness tests for the property. It does not prescribe implementation.
