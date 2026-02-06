# Bridge Crypto — IIC Conformance Scaffold

This folder contains the complete Rust workspace for the IIC (Inertial Integrity Constraint) conformance harness, as designed by BB.

## How to Use

1. Copy everything inside `bridge-crypto-scaffold/` into your `bridge-crypto` repo
2. Run `cargo test` to confirm the harness passes
3. Update the README with the new framing

---

## File Structure

```
bridge-crypto/
├── Cargo.toml                    # Workspace root
├── README.md                     # To be updated with BB's rewrite
└── crates/
    ├── iic-spec/
    │   └── README.md             # Normative clauses IIC-1/2/3
    ├── iic-core/
    │   ├── Cargo.toml
    │   └── src/lib.rs            # Traits + types (witness surface)
    ├── iic-harness/
    │   ├── Cargo.toml
    │   ├── src/lib.rs            # Conformance checks
    │   └── tests/conformance.rs  # Property tests
    └── iic-witness/
        ├── Cargo.toml
        └── src/lib.rs            # Minimal toy implementation
```

---

## Commands

```bash
# From repo root
cargo test -q

# With verbose output
cargo test
```

---

## What This Is (And Isn't)

**This IS:**
- A conformance harness that tests whether a system exhibits the IIC property
- A normative spec defining what IIC means (IIC-1, IIC-2, IIC-3)
- A minimal witness implementation to demonstrate the harness

**This IS NOT:**
- "The IIC implementation"
- A smart contract to deploy
- A pattern to copy

IIC is a property a system has or lacks. This code helps you ask the question.
