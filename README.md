# increment-decrement
A contract to explore the usage of several #[contract] in soroban-sdk, `register_contract_wasm` and expected panic substring while testing. This contract was forked from `stellar/soroban-examples`.

# Prepare
```
make build
make test
```

# Introduction:
This contract has two mods:
1) `increment.mod` that defines a `IncrementContract`
2) `decrement.mod` that defines a `DecrementContract`

Each of them have a `#[contract]` Attribute Macro. From https://docs.rs/soroban-sdk/20.0.0-rc2/soroban_sdk/attr.contract.html we know that 
```
... While there can be multiple types in a crate with #[contract], when built as a wasm file and deployed the combination of all contract functions and all contracts within a crate will be seen as a single contract....
```

Finally, the function `decrement()` defined in `DecrementContract` panics if the counter is less than `-2` with a panic string `"Decrement: Cannot be less than -2"`

# Usage of `register_contract` and `register_contract_wasm`:
To test this contract we can use `register_contract` or `register_contract_wasm`. 

With `register_contract` you should provide a Contract object, either `IncrementContract` or `DecrementContract`. And this means that the functions from one will not be known by the created contact in the test environment:

```rust
let contract_id = env.register_contract(None, IncrementContract);
let client = IncrementContractClient::new(&env, &contract_id);
assert_eq!(client.increment(), 1);
// This wont work
// assert_eq!(client.decrement(), 0);
```

In order to avoid this, we can first compile the whole contract into a `WASM` and use ``register_contract_wasm``

```rust
mod increment_decrement {
    soroban_sdk::contractimport!(file = "target/wasm32-unknown-unknown/release/increment_decrement.wasm");
    pub type IncrementDecrementContractClient<'a> = Client<'a>;
}
use increment_decrement::IncrementDecrementContractClient;

...
let contract_id = env.register_contract_wasm(None, increment_decrement::WASM);
let client = IncrementDecrementContractClient::new(&env, &contract_id);

assert_eq!(client.increment(), 1);
assert_eq!(client.decrement(), 0);
```
And this works OK.

# should_panic(expected = "my string"):
The problem is when we want to test that the contract panics with an specific string. With `register_contract` we can do:
```rust
#[test]
#[should_panic(expected = "Decrement: Cannot be less than -2")]
fn test_decrement_less_than_minus_2() {
    let env = Env::default();
    let contract_id = env.register_contract(None, DecrementContract);
    let client = DecrementContractClient::new(&env, &contract_id);
    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
    client.decrement(); // shouls panic here
}
```

And works OK

But when I want to test both Decrement and Increment I don't manage to "catch" the expected panic string

```rust

#[test]
#[should_panic(expected = "Decrement: Cannot be less than -2")]
fn test_increment_decrement_less_than_minus_2() {
    let env = Env::default();
    let contract_id = env.register_contract_wasm(None, increment_decrement::WASM);
    let client = IncrementDecrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.decrement(), 0);
    assert_eq!(client.decrement(), -1);
    assert_eq!(client.decrement(), -2);
    client.decrement();
}
```

This leads to an error, where `soroban-sdk` is not able to recognize the expected panic string:
`panic did not contain expected string`







