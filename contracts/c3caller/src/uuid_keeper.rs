use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Bytes, BytesN, Env, Symbol, Vec, Map,
    IntoVal, TryFromVal, log
};


// Constants for storage
const ADMIN: Symbol = symbol_short!("ADMIN");
const COMPLETED_SWAPIN: Symbol = symbol_short!("COMP_SWAP");
const UUID_TO_NONCE: Symbol = symbol_short!("UUIDNONCE");
const CURRENT_NONCE: Symbol = symbol_short!("CUR_NONCE");
const OPERATOR: Symbol = symbol_short!("OPERATOR");

#[contract]
pub struct C3UUIDKeeper;

#[contractimpl]
impl C3UUIDKeeper {
    // Initialize contract
    pub fn initialize(env: Env, admin: Address) {
        // Set admin
        env.storage().persistent().set(&ADMIN, &admin);
        // Initialize current nonce
        env.storage().persistent().set(&CURRENT_NONCE, &0u64);
        // Set admin as initial operator
        env.storage().persistent().set(&OPERATOR, &admin);
    }

    // Helper function to check if caller is admin
    fn check_admin(env: &Env) {
        let admin: Address = env.storage().persistent().get(&ADMIN).unwrap();
        // if admin != env.invoker() {
        //     panic!("not authorized");
        // }
        admin.require_auth();
    }

    // Helper function to check if caller is operator
    fn check_operator(env: &Env) {
        let operator: Address = env.storage().persistent().get(&OPERATOR).unwrap();
        // if operator != env.invoker() {
        //     panic!("not operator");
        // }
        operator.require_auth();
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
        Self::check_admin(&env);
        let mut completed_swapin = Self::get_completed_swapin(&env);
        completed_swapin.set(uuid, false);
        env.storage().instance().set(&COMPLETED_SWAPIN, &completed_swapin);
    }

    pub fn register_uuid(env: Env, uuid: BytesN<32>) {
        Self::check_operator(&env);
        if Self::is_completed(env.clone(), uuid.clone()) {
            panic!("UUID is already completed");
        }
        let mut completed_swapin = Self::get_completed_swapin(&env);
        completed_swapin.set(uuid, true);
        env.storage().instance().set(&COMPLETED_SWAPIN, &completed_swapin);
    }

    pub fn gen_uuid(
        env: Env,
        dapp_id: u64,
        to: Symbol,
        to_chain_id: Symbol,
        data: Bytes,
    ) -> BytesN<32> {
        Self::check_operator(&env);
        let nonce = Self::increase_nonce(&env);
        
        // Create input for hash
        

        let mut uuid_to_nonce = Self::get_uuid_to_nonce(&env);
        uuid_to_nonce.set(uuid, nonce);
        env.storage().persistent().set(&UUID_TO_NONCE, &uuid_to_nonce);

        uuid
    }

    
    pub fn calc_caller_uuid(
        env: Env,
        from: Address,
        dapp_id: u64,
        to: Symbol,
        to_chain_id: Symbol,
        data: Bytes,
    ) -> BytesN<32> {
        let nonce: u64 = env.storage().instance().get(&CURRENT_NONCE).unwrap() + 1;
        
        let mut hasher = Sha256::new();
        hasher.input(&env.current_contract_address().into_val(&env));
        hasher.input(&from.into_val(&env));
        hasher.input(&env.ledger().sequence().to_be_bytes());
        hasher.input(&dapp_id.to_be_bytes());
        hasher.input(&to.into_val(&env));
        hasher.input(&to_chain_id.into_val(&env));
        hasher.input(&nonce.to_be_bytes());
        hasher.input(&data);
        
        BytesN::from_array(&env, &hasher.result())
    }
}