#[cfg(test)]
mod test {
    use crate::c3gov_client::C3GovClient;

    use super::*;

    #[test]
    fn test_initialization() {
        let env = Env::default();
        let contract_id = env.register_test_contract(None, C3GovClient);
        
        let gov = Address::generate(&env);
        
        // Initialize contract
        env.invoke_contract::<()>(
            &contract_id,
            "initialize",
            vec![&env, gov.into_val(&env)],
        );

        // Verify gov is set
        let stored_gov: Address = env.invoke_contract(
            &contract_id,
            "get_gov",
            vec![&env],
        );
        assert_eq!(stored_gov, gov);
    }

    #[test]
    fn test_governance_change() {
        let env = Env::default();
        let contract_id = env.register_test_contract(None, C3GovClient);
        
        let initial_gov = Address::generate(&env);
        let new_gov = Address::generate(&env);
        
        // Initialize
        env.invoke_contract::<()>(
            &contract_id,
            "initialize",
            vec![&env, initial_gov.into_val(&env)],
        );

        // Change governance
        env.as_contract(&contract_id, || {
            C3GovClient::change_gov(env.clone(), new_gov.clone());
        });

        // Apply governance change
        env.as_contract(&contract_id, || {
            C3GovClient::apply_gov(env.clone());
        });

        // Verify new governance
        let final_gov: Address = env.invoke_contract(
            &contract_id,
            "get_gov",
            vec![&env],
        );
        assert_eq!(final_gov, new_gov);
    }

    #[test]
    fn test_operator_management() {
        let env = Env::default();
        let contract_id = env.register_test_contract(None, C3GovClient);
        
        let gov = Address::generate(&env);
        let operator = Address::generate(&env);
        
        // Initialize
        env.invoke_contract::<()>(
            &contract_id,
            "initialize",
            vec![&env, gov.into_val(&env)],
        );

        // Add operator
        env.as_contract(&contract_id, || {
            C3GovClient::add_operator(env.clone(), operator.clone());
        });

        // Verify operator is added
        let is_op: bool = env.invoke_contract(
            &contract_id,
            "is_operator",
            vec![&env, operator.into_val(&env)],
        );
        assert!(is_op);

        // Revoke operator
        env.as_contract(&contract_id, || {
            C3GovClient::revoke_operator(env.clone(), operator.clone());
        });

        // Verify operator is removed
        let is_op: bool = env.invoke_contract(
            &contract_id,
            "is_operator",
            vec![&env, operator.into_val(&env)],
        );
        assert!(!is_op);
    }
}