#![no_std]
use events::{LogC3CallEvent, LogExecCallEvent, LogExecFallbackEvent};
use soroban_sdk::{contract, contractimpl, log, vec, Address, Env, IntoVal, String, Symbol, Val, Vec};

mod events;
mod uuid_keeper;

trait CalculatorTrait {
    fn add(env: Env, a: i64, b: i64) -> i64;
}
#[contract]
pub struct C3Caller;

#[contractimpl]
impl C3Caller {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }

    pub fn call_remote_add(
        env: Env, 
        calculator_contract: Address, 
        value1: i64, 
        value2: i64
    ) -> u64 {
        // Prepare the contract call
        log!(&env, "Cecall called");
        let args = Vec::from_array(&env, [
            value1.into_val(&env),
            value2.into_val(&env)
        ]);
        let return_val: Val = env.invoke_contract(
            &calculator_contract,
            &Symbol::new(&env, "add"),
            args
        );
        
        return_val.get_payload()
        
    }

    // pub fn c3call(env: Env){


    //     LogC3CallEvent::emit(&env, event);
    // }

    // pub fn c3broadcast(env: Env){

    //     LogC3CallEvent::emit(&env, event);
    // }
    // pub fn execute(env: Env){

    //     LogExecCallEvent::emit(&env, event);
    // }

    // pub fn c3Fallback(env: Env){

    //     LogExecFallbackEvent::emit(&env, event);
    // }
}

#[contract]
pub struct Calculator;

#[contractimpl]
impl Calculator {
    pub fn add(env: Env, a: i64, b: i64) -> i64 {
        // Simple addition method
        log!(&env, "Calculator called");
        a + b
    }

    pub fn multiply(env: Env, a: i64, b: i64) -> i64 {
        // Another example method
        a * b
    }
}

mod test;

