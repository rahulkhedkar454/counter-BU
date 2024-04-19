use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct DecrementContract;

#[contractimpl]
impl DecrementContract {
    /// Decrement 
    pub fn decrementByOne(env: Env) -> i32 {
        // Get the current count.
        let mut count: i32 = env.storage().instance().get(&COUNTER).unwrap_or(0); 
        log!(&env, "count: {}", count);

        
      
        count -= 1;
        
        if count < -2 {
            panic!("Decrement: Cannot be less than -2");
        }
       
        env.storage().instance().set(&COUNTER, &count);

        env.storage().instance().bump(50, 100);

        count
    }
}