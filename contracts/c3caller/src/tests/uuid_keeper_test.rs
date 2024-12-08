#[cfg(test)]
mod test {
    use crate::uuid_keeper::C3UUIDKeeper;

    use super::*;
    use soroban_sdk::{testutils::Address as _, vec, Address, Bytes, BytesN, Symbol};

    #[test]
    fn test_basic_flow() {
        let env = Env::default();
        let contract_id = env.register_test_contract(None, C3UUIDKeeper);
        
        // Setup test accounts
        let admin = Address::generate(&env);
        
        // Initialize contract
        env.invoke_contract::<()>(
            &contract_id,
            "initialize",
            vec![&env, admin.into_val(&env)],
        );

        // Test UUID generation
        let dapp_id = 1u64;
        let to = Symbol::new(&env, "destination");
        let to_chain_id = Symbol::new(&env, "chain_1");
        let data = Bytes::new(&env);

        // Generate UUID (as operator)
        let uuid: BytesN<32> = env.invoke_contract(
            &contract_id,
            "gen_uuid",
            vec![
                &env,
                dapp_id.into_val(&env),
                to.into_val(&env),
                to_chain_id.into_val(&env),
                data.into_val(&env),
            ],
        );

        // Verify UUID exists
        let exists: bool = env.invoke_contract(
            &contract_id,
            "is_uuid_exist",
            vec![&env, uuid.into_val(&env)],
        );
        assert!(exists, "UUID should exist after generation");

        // Register UUID
        env.invoke_contract::<()>(
            &contract_id,
            "register_uuid",
            vec![&env, uuid.into_val(&env)],
        );

        // Verify completion
        let completed: bool = env.invoke_contract(
            &contract_id,
            "is_completed",
            vec![&env, uuid.into_val(&env)],
        );
        assert!(completed, "UUID should be marked as completed");
    }
}