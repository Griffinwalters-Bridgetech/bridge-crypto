# Treasury Attestation Flow (IIC-Aware)

This diagram illustrates how the Inertial Integrity Constraint (IIC)
changes system behavior **without adding logic or enforcement**.

It is intentionally non-executable.

---

## Scenario

A DAO treasury proposal has passed governance.

The proposal is valid.

Execution would be irreversible.

---

## Flow Overview

```
┌──────────────────────────┐
│ Proposal Submitted       │
└─────────────┬────────────┘
              ↓
┌──────────────────────────┐
│ Governance Vote          │
│ (Quorum + Threshold Met) │
└─────────────┬────────────┘
              ↓
┌──────────────────────────┐
│ Validation Succeeds      │
│ (Proposal is valid)      │
└─────────────┬────────────┘
              ↓
┌────────────────────────────────────────────┐
│ IIC-Aware State                            │
│                                            │
│ • Invocation acknowledged                  │
│ • Execution NOT assumed                    │
│ • Non-execution is a valid terminal state  │
└─────────────┬───────────────┬──────────────┘
              │               │
              │               │
              ↓               ↓
┌─────────────────────┐   ┌────────────────────────┐
│ Explicit Attestation│   │ Non-Execution          │
│ (Human Authorization)│   │ (Terminal, Correct)    │
└─────────────┬───────┘   └────────────────────────┘
              ↓
┌──────────────────────────┐
│ Execution Proceeds       │
│ (Irreversible State      │
│  Change)                 │
└──────────────────────────┘
```

---

## Key Properties Illustrated

• Validation success does **not** force continuation  
• Execution only proceeds through explicit completion  
• Non-execution is not failure, delay, or error  
• IIC does not block execution — it revokes assumption  

---

## What This Diagram Is NOT

✕ Not a smart contract flow  
✕ Not enforcement logic  
✕ Not a governance replacement  
✕ Not a pause mechanism  

This diagram represents a **system property**, not a mechanism.

---

*Bridge — governing execution before it becomes irreversible.*
