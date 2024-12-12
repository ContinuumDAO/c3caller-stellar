use soroban_sdk::{
    contract, contractimpl, symbol_short,
    Address, Env, Symbol, Vec,
    IntoVal, TryFromVal
};

// Storage keys
const GOV: Symbol = symbol_short!("GOV");
const PENDING_GOV: Symbol = symbol_short!("PEND_GOV");
const OPERATORS: Symbol = symbol_short!("OPS");
const INITIALIZED: Symbol = symbol_short!("INIT");

// Event names
const EVENT_CHANGE_GOV: Symbol = symbol_short!("CHANGEGOV");
const EVENT_APPLY_GOV: Symbol = symbol_short!("APPLY_GOV");
const EVENT_ADD_OPERATOR: Symbol = symbol_short!("ADD_OP");

#[contract]
pub struct C3GovClient;

#[contractimpl]
impl C3GovClient {
    // Initialize contract
    pub fn initialize(env: Env, gov: Address) {
        if env.storage().persistent().has(&INITIALIZED) {
            panic!("already initialized");
        }
        
        env.storage().persistent().set(&INITIALIZED, &true);
        env.storage().persistent().set(&GOV, &gov);
        env.storage().persistent().set(&OPERATORS, &Vec::<Address>::new(&env));

        // Emit initialization event
        // env.events().publish(
        //     (EVENT_APPLY_GOV,),
        //     (
        //         Address::from_string("GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF"),
        //         gov,
        //         env.ledger().timestamp(),
        //     ),
        // );
    }

    // Helper functions for authorization checks
    pub fn check_gov(env: &Env) {
        let gov: Address = env.storage().persistent().get(&GOV).unwrap();
        gov.require_auth();
    }

    pub fn check_operator(env: &Env, caller:Address) {
        let gov: Address = env.storage().persistent().get(&GOV).unwrap();
        let operators: Vec<Address> = env.storage().persistent().get(&OPERATORS).unwrap();
       
        if caller != gov && !operators.contains(&caller) {
            panic!("C3Gov: only Operator");
        }
        caller.require_auth();
    }

    // Governance functions
    pub fn change_gov(env: Env, new_gov: Address) {
        Self::check_gov(&env);
        
        let old_gov: Address = env.storage().persistent().get(&GOV).unwrap();
        env.storage().persistent().set(&PENDING_GOV, &new_gov);

        // Emit event
        env.events().publish(
            (EVENT_CHANGE_GOV,),
            (
                old_gov,
                new_gov,
                env.ledger().timestamp(),
            ),
        );
    }

    pub fn apply_gov(env: Env) {
        let pending_gov: Option<Address> = env.storage().persistent().get(&PENDING_GOV);
        let new_gov = pending_gov.expect("C3Gov: empty pendingGov");
        let old_gov: Address = env.storage().persistent().get(&GOV).unwrap();

        // Update governance
        env.storage().persistent().set(&GOV, &new_gov);
        env.storage().persistent().remove(&PENDING_GOV);

        // Emit event
        env.events().publish(
            (EVENT_APPLY_GOV,),
            (
                old_gov,
                new_gov,
                env.ledger().timestamp(),
            ),
        );
    }

    // Operator management
    fn add_operator_internal(env: &Env, op: Address) {
        if op.to_string().is_empty() {
            panic!("C3Caller: Operator is null address");
        }

        let mut operators: Vec<Address> = env.storage().persistent().get(&OPERATORS).unwrap();
        
        if operators.contains(&op) {
            panic!("C3Caller: Operator already exists");
        }

        operators.push_back(op.clone());
        env.storage().persistent().set(&OPERATORS, &operators);

        // Emit event
        env.events().publish(
            (EVENT_ADD_OPERATOR,),
            op,
        );
    }

    pub fn add_operator(env: Env, op: Address) {
        Self::check_gov(&env);
        Self::add_operator_internal(&env, op);
    }

    pub fn get_all_operators(env: Env) -> Vec<Address> {
        env.storage().instance().get(&OPERATORS).unwrap()
    }

    pub fn revoke_operator(env: Env, op: Address) {
        Self::check_gov(&env);

        let mut operators: Vec<Address> = env.storage().persistent().get(&OPERATORS).unwrap();
        
        if !operators.contains(&op) {
            panic!("C3Caller: Operator not found");
        }

        // Remove operator
        // let index = operators.iter()
        //     .position(|x| *x == op)
        //     .expect("Operator not found");
        
        // Replace with last element and pop
        let last_idx = operators.len() - 1;
        // if index != last_idx {
        //     operators.set(index, operators.get(last_idx).unwrap());
        // }
        operators.pop_back();

        // Update storage
        env.storage().persistent().set(&OPERATORS, &operators);
    }

    // View functions
    pub fn get_gov(env: Env) -> Address {
        env.storage().persistent().get(&GOV).unwrap()
    }

    pub fn get_pending_gov(env: Env) -> Option<Address> {
        env.storage().persistent().get(&PENDING_GOV)
    }

    pub fn is_operator(env: Env, op: Address) -> bool {
        let operators: Vec<Address> = env.storage().persistent().get(&OPERATORS).unwrap();
        operators.contains(&op)
    }
}