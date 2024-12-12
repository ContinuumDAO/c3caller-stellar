#[cfg(test)]
mod test {
    use crate::{c3gov_client::C3GovClient, uuid_keeper::{C3UUIDKeeper, C3UUIDKeeperClient, ADMIN, CURRENT_NONCE, OPERATOR}};

    use super::*;
    use soroban_sdk::{
        log, testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}, Address, Bytes, BytesN, Env, IntoVal, String, Symbol
    };

    fn create_test_env() -> Env {
        Env::default()
    }

    
    fn create_contract(env: &Env) -> (Address, Address, C3UUIDKeeperClient) {
        let admin = Address::generate(&env);
       
        let id =  env.register(C3UUIDKeeper, ());
        let client = C3UUIDKeeperClient::new(&env, &id);
        let c3gov_contract_id =  env.register(C3GovClient, ());
        //client.initialize(&c3gov_contract_id,&admin);
        
        
        (id, admin, client)
    }

    #[test]
    fn test_initialize() {


         let env = create_test_env();
        let gov = Address::generate(&env);
        let contract_id = env.register( C3UUIDKeeper,());
        let c3gov_contract_id =  env.register(C3GovClient, ());
        let client = C3UUIDKeeperClient::new(&env, &contract_id);

        client.initialize(&c3gov_contract_id,&gov);
        
        


        //  env.as_contract(&contract_id, ||{
        //       // Verify storage
        // let stored_admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        // let stored_nonce: u64 = env.storage().persistent().get(&CURRENT_NONCE).unwrap();
        // let stored_operator: Address = env.storage().persistent().get(&OPERATOR).unwrap();


        // assert_eq!(stored_admin, gov);
        // assert_eq!(stored_nonce, 0);
        // assert_eq!(stored_operator, gov);
            
        //  })
        

    }

    #[test]
    fn test_register_uuid() {
        // let env = create_test_env();
        // let (contract_id, admin) = create_contract(&env);
        
        // // Generate a test UUID
        // let test_uuid = BytesN::from_array(&env, &[1u8; 32]);
        
        // // Register UUID as operator (admin is operator by default)
        // env.as_contract(&contract_id, || {
        //     admin.require_auth();
        //     client.register_uuid(test_uuid.clone());
        // });

        // // Verify UUID is registered
        // assert!(client.is_completed(test_uuid.clone()));
    }

    #[test]
    //#[should_panic(expected = "UUID is already completed")]
    fn test_register_uuid_already_completed() {
        // let env = create_test_env();
        // let (contract_id, admin, client) = create_contract(&env);
        
        // let test_uuid = BytesN::from_array(&env, &[1u8; 32]);
        
        // // Register UUID first time
        // env.as_contract(&contract_id, || {
        //     admin.require_auth();
        //     client.register_uuid(test_uuid.clone());
        // });

        // // Try to register same UUID again
        // env.as_contract(&contract_id, || {
        //     admin.require_auth();
        //     client.register_uuid(test_uuid.clone());
        // });
    }

    #[test]
    fn test_gen_uuid() {

        
        let env = create_test_env();
        let (contract_id, admin, client) = create_contract(&env);
        
        // let dapp_id = 1u64;
        // let to = String::from_str(&env, "destination");
        // let to_chain_id = String::from_str(&env, "chain1");
        // let data = Bytes::from_slice(&env, &[1, 2, 3]);
        
        // let c3gov_contract_id =  env.register(C3GovClient, ());

        // client.initialize(&c3gov_contract_id, &admin);
        
        //  let uuid = client.gen_uuid(&admin,&dapp_id, &to, &to_chain_id, &data);
        // assert!(client.is_uuid_exist(&uuid));
        // Generate UUID as operator
        // env.as_contract(&contract_id, || {
        //     admin.require_auth();
        //     let uuid = //client.gen_uuid(dapp_id, to.clone(), to_chain_id.clone(), data.clone());
            
        //     // Verify UUID is registered in uuid_to_nonce mapping
           
        // });
    }

    #[test]
    fn test_revoke_swapin() {
        // let env = create_test_env();
        // let (contract_id, admin, client) = create_contract(&env);
        
        // let test_uuid = BytesN::from_array(&env, &[1u8; 32]);
        
        // // Register UUID first
        // env.as_contract(&contract_id, || {
        //     admin.require_auth();
        //     client.register_uuid(test_uuid.clone());
        // });

        // // Verify it's completed
        // assert!(client.is_completed(test_uuid.clone()));

        // // Revoke the swapin
        // env.as_contract(&contract_id, || {
        //     admin.require_auth();
        //     client.revoke_swapin(test_uuid.clone());
        // });

        // // Verify it's no longer completed
        // assert!(!client.is_completed(test_uuid));
    }

    #[test]
    //#[should_panic(expected = "UUID already exist")]
    fn test_gen_uuid_duplicate() {
        // let env = create_test_env();
        // let (contract_id, admin, client) = create_contract(&env);
        
        // let dapp_id = 1u64;
        // let to = String::from_str(&env, "destination");
        // let to_chain_id = String::from_str(&env, "chain1");
        // let data = Bytes::from_slice(&env, &[1, 2, 3]);

        // // Generate first UUID
        // env.as_contract(&contract_id, || {
        //     admin.require_auth();
        //     client.gen_uuid(dapp_id, to.clone(), to_chain_id.clone(), data.clone());
        // });

        // // Try to generate same UUID again (should panic)
        // env.as_contract(&contract_id, || {
        //     admin.require_auth();
        //     client.gen_uuid(dapp_id, to.clone(), to_chain_id.clone(), data.clone());
        // });
    }
}