use soroban_sdk::testutils::{AuthorizedFunction, AuthorizedInvocation, LedgerInfo};
#[cfg(test)]

    use soroban_sdk::{
        testutils::{Address as _, Ledger},
        Address, Env, BytesN,
    };

use crate::c3gov_client::{C3GovClient, C3GovClientClient};
    


    // Test helpers
    fn create_test_env() -> Env {
        let env = Env::default();
        env.mock_all_auths();
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
    fn test_gov_initialization() {
        let env = create_test_env();
        let (gov, _, _) = create_addresses(&env);
        
        let contract_id = env.register_contract(None, C3GovClient);
        let client = C3GovClientClient::new(&env, &contract_id);
        
        client.gov_init(&gov);
        
        assert_eq!(client.get_gov(), gov);
        assert_eq!(client.get_pending_gov(), None);
       // let operators = client.get_all_operators();
      //  assert_eq!(operators.len(), 0);
    }

 

    #[test]
    fn test_add_operator() {
        let env = create_test_env();
        let (gov, operator, _) = create_addresses(&env);
        
        let contract_id = env.register_contract(None, C3GovClient);
        let client = C3GovClientClient::new(&env, &contract_id);
        

        env.mock_all_auths();

        client.gov_init(&gov);
        
    

        client.add_operator(&operator);
        
       // assert!(client.is_operator(&operator));
       // let operators = client.get_all_operators();
       // assert_eq!(operators.len(), 1);
       // assert_eq!(operators.get(0), operator);
    }

    #[test]
    fn test_governance_change() {
        let env = create_test_env();
        let (gov, _, user) = create_addresses(&env);
        
        let contract_id = env.register_contract(None, C3GovClient);
        let client = C3GovClientClient::new(&env, &contract_id);
        
        client.gov_init(&gov);
        
        // Change governance
        client.change_gov(&user);
        assert_eq!(client.get_pending_gov(), Some(user.clone()));
        
        // Apply governance change
        client.apply_gov();
        assert_eq!(client.get_gov(), user);
        assert_eq!(client.get_pending_gov(), None);
    }

    #[test]
    fn test_revoke_operator() {
        let env = create_test_env();
        let (gov, operator, _) = create_addresses(&env);
        
        let contract_id = env.register_contract(None, C3GovClient);
        let client = C3GovClientClient::new(&env, &contract_id);
        
        client.gov_init(&gov);
        
        // Add operator
        client.add_operator(&operator);
        assert!(client.is_operator(&operator));
        
        // Revoke operator
        client.revoke_operator(&operator);
        assert!(!client.is_operator(&operator));
    }
