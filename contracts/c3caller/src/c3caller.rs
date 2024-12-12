use soroban_sdk::{
    contract, contractimpl, log, symbol_short, vec, Address, Bytes, BytesN, Env, FromVal, IntoVal, Map, String, Symbol, TryFromVal, Val, Vec,
    contracttype
};

use crate::events::{LogC3CallEvent, LogExecCallEvent};

// Storage keys
const ADMIN: Symbol = symbol_short!("ADMIN");
const UUID_KEEPER: Symbol = symbol_short!("UUID_KEEP");
const PAUSED: Symbol = symbol_short!("PAUSED");
const CONTEXT: Symbol = symbol_short!("CONTEXT");



#[derive(Clone,Debug)]
#[contracttype]
pub struct C3StellarMessage {
    to: Address,
    data: Vec<Val>,
    fallback_to: String,
    uuid: BytesN<32>,
    func:Symbol,
    from_chain_id: String,
    source_tx: String,
}



#[contract]
pub struct C3Caller;

#[contractimpl]
impl C3Caller {
    // Initialize contract
    pub fn initialize( env: Env,uuid_keeper: Address) {


        //save uuid keeper contract address in storage
        // call initGov from GovClient contract by passing the person who called the contract as admin
       
        env.storage().persistent().set(&UUID_KEEPER, &uuid_keeper);
        env.storage().persistent().set(&PAUSED, &false);
        
        
    }

    fn check_gov(env: &Env) {
       // let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
       // admin.require_auth();
       //check if the caller is gov
    }

    fn check_operator(env: &Env) {
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();
        // Note: In a full implementation, we would check operator list like in C3GovClient
        // check if he is an operator
    }

    fn check_not_paused(env: &Env) {
        let paused: bool = env.storage().persistent().get(&PAUSED).unwrap();
        if paused {
            panic!("Contract is paused");
        }
    }

    pub fn pause(env: Env) {
        Self::check_operator(&env);
        env.storage().persistent().set(&PAUSED, &true);
    }

    pub fn unpause(env: Env) {
        Self::check_operator(&env);
        env.storage().persistent().set(&PAUSED, &false);
    }

    pub fn c3_call(
        env: Env,
        dapp_id: u64,
        caller: Address,
        to: String,
        to_chain_id: String,
        data: Bytes,
        extra: Bytes,
    ) {
        Self::check_not_paused(&env);

        // Validate inputs
        if dapp_id == 0 { panic!("C3Caller: empty dappID"); }
        if to.is_empty(){ panic!("C3Caller: empty _to"); }
        if to_chain_id.is_empty() { panic!("C3Caller: empty toChainID"); }
        if data.len() == 0 { panic!("C3Caller: empty calldata"); }

        // Get UUID keeper contract
        let uuid_keeper: Address = env.storage().persistent().get(&UUID_KEEPER).unwrap();
        
        // Generate UUID through keeper contract
        let uuid: BytesN<32> = env.invoke_contract(
            &uuid_keeper,
            &Symbol::new(&env,"gen_uuid"),
            vec![
                &env,
                dapp_id.into_val(&env),
                to.into_val(&env),
                to_chain_id.into_val(&env),
                data.into_val(&env),
            ],
        );

        // Emit event

        LogC3CallEvent::emit(&env, &LogC3CallEvent {
             dapp_id,
              uuid,
               caller,
                to_chain_id, 
                to, 
                data, 
                extra
             });
       
    }

    pub fn c3_broadcast(
        env: Env,
        dapp_id: u64,
        caller: Address,
        to: Vec<String>,
        to_chain_ids: Vec<String>,
        data: Bytes,
    ) {
        Self::check_not_paused(&env);

        // Validate inputs
        if dapp_id == 0 { panic!("C3Caller: empty dappID"); }
        if to.len() == 0 { panic!("C3Caller: empty _to"); }
        if to_chain_ids.len() == 0 { panic!("C3Caller: empty toChainID"); }
        if data.len() == 0 { panic!("C3Caller: empty calldata"); }
        if to.len() != to_chain_ids.len() { panic!("C3Caller: tochains length mismatch"); }

        let uuid_keeper: Address = env.storage().persistent().get(&UUID_KEEPER).unwrap();
        let empty_extra = Bytes::new(&env);

        // Process each destination
        for i in 0..to.len() {
          
            let uuid: BytesN<32> = env.invoke_contract(
                &uuid_keeper,
               &Symbol::new(&env, "gen_uuid"),
                vec![
                    &env,
                    dapp_id.into_val(&env),
                    to.get(i).unwrap().into_val(&env),
                    to_chain_ids.get(i).unwrap().into_val(&env),
                    data.clone().into_val(&env),
                ],
            );

            // Emit event for each destination
            LogC3CallEvent::emit(&env, &LogC3CallEvent {
                dapp_id,
                 uuid,
                  caller: caller.clone(),
                   to_chain_id:to_chain_ids.get(i).unwrap(), 
                   to:to.get(i).unwrap(), 
                   data:data.clone(), 
                   extra:Bytes::new(&env),
                });
        }
    }

    // Execute function (partial implementation)
    pub fn execute(
        env: Env,
        dapp_id: u64,
        tx_sender: Address,
        message: C3StellarMessage,
    ) {
        Self::check_operator(&env);
        Self::check_not_paused(&env);

        
        if message.data.len() == 0 { 
            panic!("C3Caller: empty calldata"); 
        }

        let uuid_keeper: Address = env.storage().persistent().get(&UUID_KEEPER).unwrap();
        
        // Check if UUID is already completed
        let is_completed: bool = env.invoke_contract(
            &uuid_keeper,
            &Symbol::new(&env,"is_completed"),
            vec![&env, message.uuid.clone().into_val(&env)],
        );

        if is_completed {
            panic!("C3Caller: already completed");
        }

       
        // Execute the call (simplified as Soroban has different call mechanics)
        // In reality, this would need to be adapted to Soroban's cross-contract call patterns
        
        let reason = env.invoke_contract(
            &message.to,
             &message.func, 
            message.data.clone()
            );

            
        // Emit appropriate events based on result

        LogExecCallEvent::emit(&env, &LogExecCallEvent { 
            dapp_id,
             to: message.to,
              uuid: message.uuid,
               from_chain_id: message.from_chain_id,
                source_tx: message.source_tx,
                 data: message.data, 
                 success: true,
                  reason,
                 });
        

    
        
    }
}