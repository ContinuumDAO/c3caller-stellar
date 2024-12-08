#![no_std]
use events::{LogC3CallEvent, LogExecCallEvent, LogExecFallbackEvent};
use soroban_sdk::{contract, contractimpl, log, vec, Address, Env, IntoVal, String, Symbol, Val, Vec};

mod events;
mod uuid_keeper;
mod tests;
mod c3govenor;
mod c3gov_client;
mod c3caller;


pub use c3caller::*;
mod test;


