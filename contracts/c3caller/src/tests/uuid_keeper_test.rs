#[cfg(test)]
mod test {
    use crate::{c3gov_client::{C3GovClient, C3GovClientClient, GOV}, uuid_keeper::{C3UUIDKeeper, C3UUIDKeeperClient, ADMIN, CURRENT_NONCE, OPERATOR}};

    use super::*;
    use soroban_sdk::{
        log, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, Ledger, LedgerInfo}, Address, Bytes, BytesN, Env, IntoVal, String, Symbol
    };

    fn create_test_env() -> Env {
        let env = Env::default();
     
        // Set current ledger info
        
        env
    }

    fn create_addresses(env: &Env) -> (Address, Address, Address) {
        let gov = Address::generate(&env);
        let operator = Address::generate(&env);
        let user = Address::generate(&env);
        (gov, operator, user)
    }

    #[test]
    fn test_uuid_keeper_initialization() {
        let env = create_test_env();
        let (gov, _, _) = create_addresses(&env);
        
        // Deploy contracts
        let gov_contract_id = env.register_contract(None, C3GovClient {});
        let uuid_keeper_id = env.register_contract(None, C3UUIDKeeper {});
        
        let uuid_keeper = C3UUIDKeeperClient::new(&env, &uuid_keeper_id);
        uuid_keeper.initialize(&gov_contract_id, &gov);
    }

    #[test]
    fn test_uuid_generation() {
        let env = create_test_env();
        let (gov, operator, _) = create_addresses(&env);
        
        // Deploy and initialize contracts
        let gov_contract_id = env.register_contract(None, C3GovClient {});
        let uuid_keeper_id = env.register_contract(None, C3UUIDKeeper {});
        
        let gov_client = C3GovClientClient::new(&env, &gov_contract_id);
        let uuid_keeper = C3UUIDKeeperClient::new(&env, &uuid_keeper_id);
        
        gov_client.gov_init(&gov);
        env.mock_all_auths();
        uuid_keeper.initialize(&gov_contract_id, &gov);
        
        // Generate UUID

      let uuid = uuid_keeper.gen_uuid(
        &gov,
        &1u64,
        &"destination".into_val(&env),
        &"chain_1".into_val(&env),
        &Bytes::new(&env),
    );

       assert!(uuid_keeper.is_uuid_exist(&uuid));
       assert!(!uuid_keeper.is_completed(&uuid));
    }

    #[test]
    fn test_uuid_registration() {
        let env = create_test_env();
        let (gov, operator, _) = create_addresses(&env);
        
        // Deploy and initialize contracts
        let gov_contract_id = env.register_contract(None, C3GovClient {});
        let uuid_keeper_id = env.register_contract(None, C3UUIDKeeper {});
        
        let gov_client = C3GovClientClient::new(&env, &gov_contract_id);
        let uuid_keeper = C3UUIDKeeperClient::new(&env, &uuid_keeper_id);
        
        gov_client.gov_init(&gov);
        env.mock_all_auths();
        uuid_keeper.initialize(&gov_contract_id, &gov);
        

        let uuid = uuid_keeper.gen_uuid(
                    &gov,
                    &1u64,
                    &"destination".into_val(&env),
                    &"chain_1".into_val(&env),
                    &Bytes::new(&env),
                );
        
        
        uuid_keeper.register_uuid(&gov, &uuid);

      assert!(uuid_keeper.is_completed(&uuid));
      
    }

    #[test]
    #[should_panic(expected = "UUID is already completed")]
    fn test_double_registration() {
        let env = create_test_env();
        let (gov, operator, _) = create_addresses(&env);
        
        // Deploy and initialize contracts
        let gov_contract_id = env.register_contract(None, C3GovClient {});
        let uuid_keeper_id = env.register_contract(None, C3UUIDKeeper {});
        
        let gov_client = C3GovClientClient::new(&env, &gov_contract_id);
        let uuid_keeper = C3UUIDKeeperClient::new(&env, &uuid_keeper_id);
        
        gov_client.gov_init(&gov);
        env.mock_all_auths();
        uuid_keeper.initialize(&gov_contract_id, &gov);
    
        // Generate and register UUID twice
        
            let uuid = uuid_keeper.gen_uuid(
                &gov,
                &1u64,
                &"destination".into_val(&env),
                &"chain_1".into_val(&env),
                &Bytes::new(&env),
            );
            
            uuid_keeper.register_uuid(&gov, &uuid);
            uuid_keeper.register_uuid(&gov, &uuid); // Should panic
       
    }

    #[test]
    fn test_revoke_swapin() {
        let env = create_test_env();
        let (gov, operator, _) = create_addresses(&env);
        
        // Deploy and initialize contracts
        let gov_contract_id = env.register_contract(None, C3GovClient {});
        let uuid_keeper_id = env.register_contract(None, C3UUIDKeeper {});
        
        let gov_client = C3GovClientClient::new(&env, &gov_contract_id);
        let uuid_keeper = C3UUIDKeeperClient::new(&env, &uuid_keeper_id);
        
        gov_client.gov_init(&gov);
        env.mock_all_auths();
        uuid_keeper.initialize(&gov_contract_id, &gov);
        
        // Add operator
        gov_client.add_operator(&operator);
        
        // Generate and register UUID
        let uuid = uuid_keeper.gen_uuid(
            &operator,
            &1u64,
            &"destination".into_val(&env),
            &"chain_1".into_val(&env),
            &Bytes::new(&env),
        );
           
            
     uuid_keeper.register_uuid(&operator, &uuid);
      
        // // Revoke swapin
      uuid_keeper.revoke_swapin(&uuid);
        
     assert!(!uuid_keeper.is_completed(&uuid));
    }
    
    
}