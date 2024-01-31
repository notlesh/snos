#[cfg(test)]
mod tests {

    use cairo_vm::{
        serde::deserialize_program::ApTracking,
        types::exec_scope::ExecutionScopes,
    };
    use num_bigint::BigInt;

    use crate::hints::*;

    #[test]
    fn test_is_on_curve() {
        let mut vm = VirtualMachine::new(false);
        // TODO: allocate memory for ids.is_on_curve (?)

        let ids_data = Default::default();
        let ap_tracking = ApTracking::default();

        let mut exec_scopes = ExecutionScopes::new();

        let y = BigInt::from(1234);
        let y_square_int = y.clone() * y.clone();

        exec_scopes.insert_value("y", y);
        exec_scopes.insert_value("y_square_int", y_square_int);

        // TODO: use an appropriate constant for SECP_P. Also see TODO in `fn is_on_curve` -- should it be in
        // exec_scopes to begin with?
        use std::str::FromStr;
        let SECP_P = BigInt::from_str("115792089237316195423570985008687907853269984665640564039457584007908834671663").unwrap();
        exec_scopes.insert_value("SECP_P", SECP_P);

        is_on_curve(&mut vm, &mut exec_scopes, &ids_data, &ap_tracking, &Default::default())
            .expect("is_on_curve() failed");

        // TODO: is_on_curve should exist in ids_data, not exec_scopes
        let is_on_curve: Felt252 = ids_data.get("is_on_curve")
            .expect("is_on_curve local var should be defined in hint");
        assert_eq!(is_on_curve, 1.into());
    }
}