#[cfg(test)]
mod test {
    use soroban_sdk::{testutils::BytesN, Address, Bytes, Vec};

    use super::*;
    
    #[test]
    fn test_basic_flow() {
        let env = Env::default();
        let contract_id = env.register_test_contract(None, C3Governor);
        
        // Setup test accounts
        let admin = Address::generate(&env);
        
        // Initialize contract
        env.invoke_contract::<()>(
            &contract_id,
            "initialize",
            vec![&env, admin.into_val(&env)],
        );

        // Create test data
        let test_data = Bytes::from_slice(&env, &[1, 2, 3]);
        let nonce = BytesN::from_array(&env, &env.crypto().sha256(&test_data));

        // Test sending params (as admin)
        env.as_contract(&contract_id, || {
            C3Governor::send_params(
                env.clone(),
                test_data.clone(),
                nonce.clone()
            );
        });

        // Verify proposal exists
        let (stored_data, failed) = env.invoke_contract(
            &contract_id,
            "get_proposal_data",
            vec![
                &env,
                nonce.into_val(&env),
                0u32.into_val(&env),
            ],
        );

        assert_eq!(stored_data, test_data);
        assert_eq!(failed, false);
    }

    #[test]
    fn test_multi_params() {
        let env = Env::default();
        let contract_id = env.register_test_contract(None, C3Governor);
        
        let admin = Address::generate(&env);
        env.invoke_contract::<()>(
            &contract_id,
            "initialize",
            vec![&env, admin.into_val(&env)],
        );

        // Create test data
        let mut test_data = Vec::new(&env);
        test_data.push_back(Bytes::from_slice(&env, &[1, 2, 3]));
        test_data.push_back(Bytes::from_slice(&env, &[4, 5, 6]));
        
        let nonce = BytesN::from_array(&env, &env.crypto().sha256(&test_data.get(0).unwrap()));

        // Test sending multiple params
        env.as_contract(&contract_id, || {
            C3Governor::send_multi_params(
                env.clone(),
                test_data.clone(),
                nonce.clone()
            );
        });

        // Verify proposals exist
        for i in 0..2 {
            let (stored_data, failed) = env.invoke_contract(
                &contract_id,
                "get_proposal_data",
                vec![
                    &env,
                    nonce.into_val(&env),
                    i.into_val(&env),
                ],
            );

            assert_eq!(stored_data, test_data.get(i).unwrap());
            assert_eq!(failed, false);
        }
    }
}