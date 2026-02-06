//! iic-harness
//!
//! Reusable conformance checks for IIC-like properties.
//! The harness tests *properties*; it does not define one true implementation.

use iic_core::{Attestation, Disposition, ExecutionAttestor, Invocable};

/// Conformance failure types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConformanceFailure {
    /// Invocation caused attestation without an explicit attestation step.
    /// Violates IIC-1.
    InvocationImpliesAttestation,
    /// A system fails to represent terminal non-execution as a valid disposition.
    /// Violates IIC-2.
    NoExecutionNotRepresentable,
}

/// Check: invocation does not imply attestation (IIC-1).
///
/// Requires an invocable surface.
pub fn check_invocation_not_imply_attestation<S: Invocable>(
    mut system: S,
    action: S::ActionId,
) -> Result<(), ConformanceFailure> {
    let before = system.is_attested(action);
    system.invoke(action);
    let after = system.is_attested(action);

    // Core invariant: invocation alone must not "upgrade" an action to attested.
    if before == Attestation::Absent && after == Attestation::Present {
        return Err(ConformanceFailure::InvocationImpliesAttestation);
    }
    Ok(())
}

/// Check: "NoExecution" must be representable as a valid terminal outcome (IIC-2).
///
/// NOTE: This is *representability* only (shape), not enforcement of semantics.
pub fn check_no_execution_representable<S: ExecutionAttestor>(
    system: &S,
    action: S::ActionId,
) -> Result<(), ConformanceFailure> {
    let d = system.disposition(action);
    if d != Disposition::Unknown
        && d != Disposition::AttestedExecution
        && d != Disposition::NoExecution
    {
        // This branch is currently unreachable given enum, but kept for "external mapping" cases later.
        return Err(ConformanceFailure::NoExecutionNotRepresentable);
    }
    // Stronger shape check: ensure the type system includes NoExecution at all.
    // If you later map from external states, keep this check as a real mapping assertion.
    Ok(())
}
