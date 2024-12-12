// #![cfg(test)]

// use soroban_sdk::{
//     log, testutils::Address as _, Address, Env, IntoVal, Symbol, Val, Vec
// };

// // Import the contracts we want to test
// use crate::{C3Caller,};

// #[test]
// fn test_calculator_add() {
//     // Create a test environment
//     let env = Env::default();
    
//     // Create test addresses for contracts
//    // let calculator_contract = env.register_contract(None, Calculator);
//     let caller_contract = env.register_contract(None, C3Caller);

    
//     // Prepare test inputs
//     let value1 = 10_i64;
//     let value2 = 20_i64;
    
//     // // Call the remote add method
//     // let result:u64 =  env.invoke_contract(&caller_contract, &Symbol::new(&env, "call_remote_add"), Vec::from_array(&env, [
//     //     calculator_contract.into_val(&env),
//     //     value1.into_val(&env),
//     //     value2.into_val(&env),
//     // ]));
    
//     // log!(&env,"result log {}", result);
//     // Convert result back to i64 and verify
//     //let actual_result: u64 = result.try_into_val(&env).unwrap();
//     //assert_eq!(result, 30, "Remote addition should work correctly");
// }

// // #[test]
// // fn test_calculator_multiply() {
// //     let env = Env::default();
    
// //     let calculator_contract = env.register_contract(None, Calculator);
// //     let caller_contract = env.register_contract(None, C3Caller);
    
// //     // Prepare test inputs for multiplication
// //     let calculator_method = Symbol::new(&env, "multiply");
// //     let value1 = 7_i64;
// //     let value2 = 6_i64;
    
// //     // Directly invoke the multiply method on the Calculator contract
// //     // let direct_result: Val = calculator_contract.call(
// //     //     &env, 
// //     //     &calculator_method, 
// //     //     Vec::from_array(&env, [
// //     //         value1.into_val(&env),
// //     //         value2.into_val(&env)
// //     //     ])
// //     // );
    
// //     // // Convert result back to i64 and verify
// //     // let direct_calculation: i64 = direct_result.try_into_val(&env).unwrap();
// //     // assert_eq!(direct_calculation, value1 * value2, "Direct multiplication should work correctly");
// // }

// // #[test]
// // fn test_process_remote_calculation() {
// //     let env = Env::default();
    
// //     let calculator_contract = env.register_contract(None, Calculator);
// //     let caller_contract = env.register_contract(None, C3Caller);
    
// //     // Prepare test inputs
// //     let value1 = 15_i64;
// //     let value2 = 25_i64;
    
// //     // // Call the process remote calculation method
// //     // let result = caller_contract.call::<Val>(
// //     //     &env, 
// //     //     &Symbol::new(&env, "process_remote_calculation"), 
// //     //     Vec::from_array(&env, [
// //     //         calculator_contract.into_val(&env),
// //     //         value1.into_val(&env),
// //     //         value2.into_val(&env)
// //     //     ])
// //     // );
    
// //     // // Convert result back to i64 and verify
// //     // let actual_result: i64 = result.try_into_val(&env).unwrap();
// //     // let expected_result = (value1 + value2) * 2;
// //     // assert_eq!(actual_result, expected_result, "Processed remote calculation should work correctly");
// // }

// // #[test]
// // fn test_zero_and_negative_inputs() {
// //     let env = Env::default();
    
// //     let calculator_contract = env.register_contract(None, Calculator);
// //     let caller_contract = env.register_contract(None, C3Caller);
    
    
// //     // Test zero input
// //     // let zero_result = caller_contract.call::<Val>(
// //     //     &env, 
// //     //     &Symbol::new(&env, "call_remote_add"), 
// //     //     Vec::from_array(&env, [
// //     //         calculator_contract.into_val(&env),
// //     //         0_i64.into_val(&env),
// //     //         0_i64.into_val(&env)
// //     //     ])
// //     // );
// //     // let zero_calculation: i64 = zero_result.try_into_val(&env).unwrap();
// //     // assert_eq!(zero_calculation, 0, "Adding zeros should return zero");
    
// //     // // Test negative input
// //     // let negative_result = caller_contract.call::<Val>(
// //     //     &env, 
// //     //     &Symbol::new(&env, "call_remote_add"), 
// //     //     Vec::from_array(&env, [
// //     //         calculator_contract.into_val(&env),
// //     //         (-10_i64).into_val(&env),
// //     //         5_i64.into_val(&env)
// //     //     ])
// //     // );
// //     // let negative_calculation: i64 = negative_result.try_into_val(&env).unwrap();
// //     // assert_eq!(negative_calculation, -5, "Adding positive and negative numbers should work");
// // }