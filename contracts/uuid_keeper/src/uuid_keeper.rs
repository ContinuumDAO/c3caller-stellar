use soroban_sdk::{
    contract, contractimpl, log, symbol_short, xdr::ToXdr, Address, Bytes, BytesN, Env, IntoVal, Map, String, Symbol, TryFromVal, Vec
};




// Constants for storage
pub const ADMIN: Symbol = symbol_short!("ADMIN");
pub const COMPLETED_SWAPIN: Symbol = symbol_short!("COMP_SWAP");
pub const UUID_TO_NONCE: Symbol = symbol_short!("UUIDNONCE");
pub const CURRENT_NONCE: Symbol = symbol_short!("CUR_NONCE");
pub const OPERATOR: Symbol = symbol_short!("OPERATOR");
pub const C3GOV_CLIENT:Symbol = symbol_short!("GOV_ADDR");
const INITIALIZED: Symbol = symbol_short!("INIT");

mod C3GovClient {
    soroban_sdk::contractimport!(
        file ="/Users/davidkathoh/projects/c3caller/target/wasm32-unknown-unknown/release/c3gov_client.wasm"
    );
}

#[contract]
pub struct C3UUIDKeeper;

#[contractimpl]
impl C3UUIDKeeper {
    // Initialize contract
    pub fn initialize(env: Env,c3gov_contract_id:Address ,gov: Address) {

        if !env.storage().persistent().has(&INITIALIZED) {
            env.storage().persistent().set(&INITIALIZED, &true);

        let admin_client = C3GovClient::Client::new(&env,&c3gov_contract_id);
         admin_client.gov_init(&gov);
        // Initialize current nonce
        env.storage().persistent().set(&CURRENT_NONCE, &0u64);
       env.storage().persistent().set(&C3GOV_CLIENT, &c3gov_contract_id);

    }
    }
   

    // Helper function to check if caller is operator
    fn check_operator(env: &Env,caller:Address) {
        let c3gov_contract_id:Address = env.storage().persistent().get(&C3GOV_CLIENT).unwrap();
        let gov_client = C3GovClient::Client::new(&env,&c3gov_contract_id);
        caller.require_auth();
        gov_client.check_operator(&caller);
    }

    //helper function to check if caller is gov
     fn check_gov(env: &Env){
         let c3gov_contract_id:Address = env.storage().persistent().get(&C3GOV_CLIENT).unwrap();
        let gov_client = C3GovClient::Client::new(&env,&c3gov_contract_id);
         let gov = gov_client.get_gov();
         gov.require_auth();
        
        
     }

    // Get storage maps
    fn get_completed_swapin(env: &Env) -> Map<BytesN<32>, bool> {
        env.storage().persistent().get(&COMPLETED_SWAPIN)
            .unwrap_or(Map::new(env))
    }

    fn get_uuid_to_nonce(env: &Env) -> Map<BytesN<32>, u64> {
        env.storage().persistent().get(&UUID_TO_NONCE)
            .unwrap_or(Map::new(env))
    }

    // Increment nonce and return new value
    fn increase_nonce(env: &Env) -> u64 {
        let current: u64 = env.storage().persistent().get(&CURRENT_NONCE).unwrap();
        let new_nonce = current + 1;
        env.storage().persistent().set(&CURRENT_NONCE, &new_nonce);
        new_nonce
    }

    // External functions
    pub fn is_uuid_exist(env: Env, uuid: BytesN<32>) -> bool {
        let uuid_to_nonce = Self::get_uuid_to_nonce(&env);
        uuid_to_nonce.contains_key(uuid)
    }

    pub fn is_completed(env: Env, uuid: BytesN<32>) -> bool {
        let completed_swapin = Self::get_completed_swapin(&env);
        completed_swapin.get(uuid).unwrap_or(false)
    }

    pub fn revoke_swapin(env: Env, uuid: BytesN<32>) {
        Self::check_gov(&env);
        let mut completed_swapin = Self::get_completed_swapin(&env);
        completed_swapin.set(uuid, false);
        env.storage().persistent().set(&COMPLETED_SWAPIN, &completed_swapin);
    }

    pub fn register_uuid(env: Env,caller:Address, uuid: BytesN<32>) {
        Self::check_operator(&env,caller);
        if Self::is_completed(env.clone(), uuid.clone()) {
            panic!("UUID is already completed");
        }
        let mut completed_swapin = Self::get_completed_swapin(&env);
        completed_swapin.set(uuid, true);
        env.storage().persistent().set(&COMPLETED_SWAPIN, &completed_swapin);
    }

    pub fn gen_uuid(
        env: Env,
        caller:Address,
        dapp_id: u64,
        to: String,
        to_chain_id: String,
        data: Bytes,
    ) -> BytesN<32> {
        
        Self::check_operator(&env, caller.clone());
        let nonce = Self::increase_nonce(&env);
          
          let mut  concat = Bytes::new(&env);
          concat.append(&env.current_contract_address().to_xdr(&env));
          concat.append(&caller.to_xdr(&env));
          concat.append(&env.ledger().network_id().to_xdr(&env));
          concat.append(&dapp_id.to_be_bytes().to_xdr(&env));
          concat.append(&to.to_xdr(&env));
          concat.append(&to_chain_id.to_xdr(&env));
          concat.append(&nonce.to_be_bytes().to_xdr(&env));
          concat.append(&data);

        
         let uuid = env.crypto().sha256(&concat).to_bytes();
         

        if Self::is_uuid_exist(env.clone(), uuid.clone()) {
             panic!("UUID already exist")
        }
        

        let mut uuid_to_nonce = Self::get_uuid_to_nonce(&env);
        uuid_to_nonce.set(uuid.clone(), nonce);
        env.storage().persistent().set(&UUID_TO_NONCE, &uuid_to_nonce);
        uuid
    }

    
    // pub fn calc_caller_uuid(
    //     env: Env,
    //     from: Address,
    //     dapp_id: u64,
    //     to: Symbol,
    //     to_chain_id: Symbol,
    //     data: Bytes,
    // ) -> BytesN<32> {
    //     let nonce: u64 = env.storage().instance().get(&CURRENT_NONCE).unwrap() + 1;
        
    //     let mut hasher = Sha256::new();
    //     hasher.input(&env.current_contract_address().into_val(&env));
    //     hasher.input(&from.into_val(&env));
    //     hasher.input(&env.ledger().sequence().to_be_bytes());
    //     hasher.input(&dapp_id.to_be_bytes());
    //     hasher.input(&to.into_val(&env));
    //     hasher.input(&to_chain_id.into_val(&env));
    //     hasher.input(&nonce.to_be_bytes());
    //     hasher.input(&data);
        
    //     BytesN::from_array(&env, &hasher.result())
    // }
}