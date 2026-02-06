//! iic-core
//!
//! This crate does NOT implement the IIC invariant.
//! It defines minimal, non-prescriptive surfaces that *may* act as witnesses.
//!
//! IIC is a property: invocation does not imply obligation to execute.

/// Terminal disposition of an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Disposition {
    /// No terminal outcome yet (or unknown to the observer).
    Unknown = 0,
    /// Explicit attestation exists that execution occurred (or will occur under an attested commitment).
    AttestedExecution = 1,
    /// Explicit terminal non-execution. This must be representable and treated as valid.
    NoExecution = 2,
}

/// Whether an explicit attestation exists for an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Attestation {
    Absent,
    Present,
}

/// A minimal trait representing an *execution attestation surface*.
///
/// - This is not "IIC."
/// - This is a witness surface against which IIC conformance can be evaluated.
pub trait ExecutionAttestor {
    type ActionId: Copy + core::fmt::Debug + Eq;

    /// Returns whether an explicit attestation exists for an action.
    fn is_attested(&self, action: Self::ActionId) -> Attestation;

    /// Returns the current disposition for an action.
    fn disposition(&self, action: Self::ActionId) -> Disposition;
}

/// Optional trait: an invocable surface for testability.
/// Not all systems will expose this, but harnesses can use it when present.
pub trait Invocable: ExecutionAttestor {
    /// "Invoke" / "request" / "propose" an action.
    ///
    /// The IIC property requires that invocation MUST NOT by itself imply attestation.
    fn invoke(&mut self, action: Self::ActionId);
}
