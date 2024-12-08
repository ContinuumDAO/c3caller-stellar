use soroban_sdk::{Address, Bytes, BytesN, Env, String, Symbol, Vec};

#[derive(Clone, Debug)]
pub struct LogC3CallEvent {
    pub dapp_id: u64,
    pub uuid: BytesN<32>,
    pub caller: Address,
    pub to_chain_id: String,
    pub to: String,
    pub data: Bytes,
    pub extra: Bytes,
}

impl LogC3CallEvent {
    pub fn emit(env: &Env, event: &LogC3CallEvent) {
        env.events().publish(
            (Symbol::new(env, "LogC3Call"), 
             event.dapp_id, 
             event.uuid.clone()),
            (
                event.caller.clone(), 
                event.to_chain_id.clone(), 
                event.to.clone(),
                event.data.clone(), 
                event.extra.clone()
            )
        );
    }
}


// Fallback Call Event
#[derive(Clone, Debug)]
pub struct LogFallbackCallEvent {
    pub dapp_id: u64,
    pub uuid: BytesN<32>,
    pub to: String,
    pub data: Bytes,
    pub reasons: Bytes,
}

impl LogFallbackCallEvent {
    pub fn emit(env: &Env, event: &LogFallbackCallEvent) {
        env.events().publish(
            (Symbol::new(env, "LogFallbackCall"), 
             event.dapp_id, 
             event.uuid.clone()),
            (
                event.to.clone(), 
                event.data.clone(), 
                event.reasons.clone()
            )
        );
    }
}

// Execution Call Event
#[derive(Clone, Debug)]
pub struct LogExecCallEvent {
    pub dapp_id: u64,
    pub to: Address,
    pub uuid: BytesN<32>,
    pub from_chain_id: String,
    pub source_tx: String,
    pub data: Bytes,
    pub success: bool,
    pub reason: Bytes,
}

impl LogExecCallEvent {
    pub fn emit(env: &Env, event: &LogExecCallEvent) {
        env.events().publish(
            (Symbol::new(env, "LogExecCall"), 
             event.dapp_id, 
             event.to.clone(),
             event.uuid.clone()),
            (
                event.from_chain_id.clone(), 
                event.source_tx.clone(), 
                event.data.clone(),
                event.success,
                event.reason.clone()
            )
        );
    }
}

// Execution Fallback Event
#[derive(Clone, Debug)]
pub struct LogExecFallbackEvent {
    pub dapp_id: u64,
    pub to: Address,
    pub uuid: BytesN<32>,
    pub from_chain_id: String,
    pub source_tx: String,
    pub data: Bytes,
    pub reason: Bytes,
}

impl LogExecFallbackEvent {
    pub fn emit(env: &Env, event: &LogExecFallbackEvent) {
        env.events().publish(
            (Symbol::new(env, "LogExecFallback"), 
             event.dapp_id, 
             event.to.clone(),
             event.uuid.clone()),
            (
                event.from_chain_id.clone(), 
                event.source_tx.clone(), 
                event.data.clone(),
                event.reason.clone()
            )
        );
    }
}


   
   
   
