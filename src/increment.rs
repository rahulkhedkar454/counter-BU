
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn incrementByOne(env: Env) -> i32 {
        // Get the current count.
        let mut count: i32 = env.storage().instance().get(&COUNTER).unwrap_or(0); // If no value set, assume 0.
        log!(&env, "count: {}", count);

       
        count += 1;

      
        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().bump(50, 100);
        count
    }
}