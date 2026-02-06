use iic_core::{Attestation, Disposition, ExecutionAttestor, Invocable};
use iic_harness::{check_invocation_not_imply_attestation, check_no_execution_representable};
use iic_witness::WitnessSystem;
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_invocation_does_not_imply_attestation(action in any::<u64>()) {
        let sys = WitnessSystem::new();
        let res = check_invocation_not_imply_attestation(sys, action);
        prop_assert!(res.is_ok());
    }

    #[test]
    fn prop_no_execution_is_representable(action in any::<u64>()) {
        let sys = WitnessSystem::new();
        let res = check_no_execution_representable(&sys, action);
        prop_assert!(res.is_ok());
    }
}

#[test]
fn test_witness_invoke_does_not_attest() {
    let mut sys = WitnessSystem::new();
    let action = 42u64;
    
    assert_eq!(sys.is_attested(action), Attestation::Absent);
    sys.invoke(action);
    assert_eq!(sys.is_attested(action), Attestation::Absent);
}

#[test]
fn test_witness_explicit_attestation_required() {
    let mut sys = WitnessSystem::new();
    let action = 42u64;
    
    sys.invoke(action);
    assert_eq!(sys.disposition(action), Disposition::Unknown);
    
    sys.explicit_attest_execution(action);
    assert_eq!(sys.disposition(action), Disposition::AttestedExecution);
    assert_eq!(sys.is_attested(action), Attestation::Present);
}

#[test]
fn test_witness_no_execution_is_valid() {
    let mut sys = WitnessSystem::new();
    let action = 42u64;
    
    sys.invoke(action);
    sys.explicit_declare_no_execution(action);
    
    assert_eq!(sys.disposition(action), Disposition::NoExecution);
    assert_eq!(sys.is_attested(action), Attestation::Absent);
}
