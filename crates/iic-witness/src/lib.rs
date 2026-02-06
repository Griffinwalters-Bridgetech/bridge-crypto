//! iic-witness
//!
//! A minimal witness implementation used to run the harness.
//! 
//! ⚠️  THIS IS NOT "THE IIC IMPLEMENTATION"  ⚠️
//! 
//! It's a toy system that should pass the conformance tests.
//! Do not deploy this. Do not fork this as "IIC."
//! It exists only to demonstrate that the harness runs.

use std::collections::HashMap;
use iic_core::{Attestation, Disposition, ExecutionAttestor, Invocable};

/// A minimal witness system for IIC conformance testing.
/// 
/// In this toy model:
/// - `invoke()` records that an action was invoked
/// - invocation does NOT produce attestation (this is the IIC property)
/// - attestations require explicit calls to `explicit_attest_execution()`
/// - non-execution requires explicit calls to `explicit_declare_no_execution()`
#[derive(Default, Clone)]
pub struct WitnessSystem {
    invoked: HashMap<u64, bool>,
    attested: HashMap<u64, bool>,
    disposition: HashMap<u64, Disposition>,
}

impl WitnessSystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Explicitly attest that an action will execute.
    /// 
    /// This is the step that upgrades "invoked" → "will execute."
    /// IIC-3 requires this to be explicit and checkable.
    pub fn explicit_attest_execution(&mut self, action: u64) {
        self.attested.insert(action, true);
        self.disposition.insert(action, Disposition::AttestedExecution);
    }

    /// Explicitly declare that an action will NOT execute.
    /// 
    /// This is a valid terminal outcome (IIC-2).
    /// Non-execution is not an error — it's a correct, complete state.
    pub fn explicit_declare_no_execution(&mut self, action: u64) {
        self.disposition.insert(action, Disposition::NoExecution);
        // Critically: still not "attested execution."
        self.attested.insert(action, false);
    }
}

impl ExecutionAttestor for WitnessSystem {
    type ActionId = u64;

    fn is_attested(&self, action: Self::ActionId) -> Attestation {
        match self.attested.get(&action).copied().unwrap_or(false) {
            true => Attestation::Present,
            false => Attestation::Absent,
        }
    }

    fn disposition(&self, action: Self::ActionId) -> Disposition {
        self.disposition.get(&action).copied().unwrap_or(Disposition::Unknown)
    }
}

impl Invocable for WitnessSystem {
    fn invoke(&mut self, action: Self::ActionId) {
        // Invocation records a request but MUST NOT upgrade attestation.
        // This is the core IIC-1 property.
        self.invoked.insert(action, true);
        // Note: we do NOT touch self.attested or self.disposition here.
    }
}
