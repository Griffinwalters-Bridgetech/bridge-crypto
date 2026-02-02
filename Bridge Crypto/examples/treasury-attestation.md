# Treasury Attestation

### Illustrative Flow: IIC Applied to Treasury Movements (Non-Executable)

This document illustrates how the Inertial Integrity Constraint (IIC) applies to DAO treasury actions, where execution is high-impact, irreversible, and frequently exploited through valid governance paths.

The purpose is not to propose a mechanism, but to show how IIC changes what the system is allowed to assume after authorization.

---

## Scenario Overview

* A DAO controls a treasury holding significant on-chain assets
* Treasury actions are authorized through governance or multisig
* Historically, authorization implies execution

This flow introduces an IIC boundary between authorization and asset movement.

---

## Baseline (Without IIC)

```
Treasury Proposal Created
        ↓
Authorization Achieved
(vote / multisig / threshold)
        ↓
Execution Triggered
        ↓
Assets Transferred
        ↓
Irreversible State Change
```

**Implicit assumption:**

> Authorized treasury movement should execute.

---

## IIC-Aware Treasury Flow

```
Treasury Proposal Created
        ↓
Authorization Achieved (Valid)
        ↓
────────────────────────
IIC Boundary
────────────────────────
        ↓
IIC-Aware Treasury State (RED Territory)
        ↓
Explicit Continuation Required
        ↓
┌───────────────────┬────────────────────┐
│                   │                    │
│ Execute Transfer  │ Do Not Transfer    │
│ (Attested)        │ (Terminal State)   │
│                   │                    │
└───────────────────┴────────────────────┘
```

---

## Properties of the IIC-Aware Treasury State

* Authorization is complete and valid
* No thresholds were violated
* No rules were broken
* Execution is possible, but not presumed

The treasury is not "paused."
It is in a state where movement requires explicit continuation.

---

## Explicit Continuation (Illustrative)

Continuation may involve:

* Human attestation of intent
* Time-separated confirmation
* Cross-domain verification (off-chain context)
* Re-affirmation of destination, amount, and impact

**IIC does not define the attestation method.**
It defines that some form of explicit continuation must exist.

---

## Non-Execution as a Valid Outcome

Under IIC:

* Choosing not to move funds is a correct terminal state
* The system does not treat inaction as error or failure
* Treasury integrity is preserved without rollback or emergency controls

**Authorization success does not force asset movement.**

---

## Why This Matters

Many treasury exploits and losses followed this pattern:

1. Authorization obtained legitimately
2. Execution proceeded automatically
3. Contextual risk was discovered after assets moved

In those cases:

* The authorization was valid
* Execution was technically correct
* Non-execution would have been the correct outcome

**The system had no way to represent that correctness.**

---

## What This Does Not Change

This flow does not:

* Override governance authority
* Introduce discretionary veto power
* Modify quorum, thresholds, or roles
* Prevent execution
* Encode policy logic

The DAO retains full control over its treasury.

**IIC only removes the assumption that authorization equals obligation.**

---

## Structural Difference Introduced by IIC

Two DAOs may use identical treasury contracts.

The difference is not capability —
it is posture.

A treasury system with IIC can represent:

* "This transfer is authorized"
* "Execution is possible"
* "Not transferring is still correct"

A treasury system without IIC cannot.

---

## Purpose of This Example

This document exists to:

* Show how IIC applies beyond governance votes
* Demonstrate non-execution as integrity, not failure
* Clarify why IIC must precede implementation choices

**This is architecture, not enforcement.**

---

*Bridge — governing execution before it becomes irreversible.*
