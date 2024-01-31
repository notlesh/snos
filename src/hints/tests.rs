#[cfg(test)]
mod tests {

    use cairo_vm::{
        serde::deserialize_program::ApTracking,
        types::exec_scope::ExecutionScopes,
        utils::test_utils::*,
    };
    use num_bigint::BigInt;

    use crate::hints::*;

    #[test]
    fn test_is_on_curve() {
        let mut vm = vm!();
        let ids_data = ids_data![];
        let ap_tracking = ApTracking::default();

        let mut exec_scopes = ExecutionScopes::new();

        let y = BigInt::from(1234);
        let y_square_int = y * y;

        exec_scopes.insert_value("y", y);
        exec_scopes.insert_value("y_square_int", y_square_int);

        is_on_curve(&mut vm, &mut exec_scopes, &ids_data, &ap_tracking, &Default::default())
            .expect("is_on_curve() failed");

        let is_on_curve: Felt252 = exec_scopes.get("is_on_curve")?;
        assert_eq!(is_on_curve, 1.into());
    }
}