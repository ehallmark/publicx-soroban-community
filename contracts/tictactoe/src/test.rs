#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation},
    Address, Env, IntoVal
};

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(Contract, ());
    let client = ContractClient::new(&env, &contract_id);

    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    // alice start's the game
    assert_eq!(
        client.start(&alice, &bob),
        0
    );

    // verify signature
    assert_eq!(
        env.auths(),
        std::vec![(
            // Address for which authorization check is performed
            alice.clone(),
            // Invocation tree that needs to be authorized
            AuthorizedInvocation {
                // Function that is authorized. Can be a contract function or
                // a host function that requires authorization.
                function: AuthorizedFunction::Contract((
                    // Address of the called contract
                    contract_id.clone(),
                    // Name of the called function
                    symbol_short!("start"),
                    // Arguments used to call `increment` (converted to the env-managed vector via `into_val`)
                    (alice.clone(), bob.clone()).into_val(&env),
                )),
                // The contract doesn't call any other contracts that require
                // authorization,
                sub_invocations: std::vec![]
            }
        )]
    );

    // starting game again should fail
    assert_eq!(
        client.start(&alice, &bob),
        1
    
    );
    
    // alice goes middle
    assert_eq!(
        client.play(&alice, &4),
        0
    );

    // bob can't also go center center
    assert_eq!(
        client.play(&bob, &4),
        1
    );

    // bob can't go off board
    assert_eq!(
        client.play(&bob, &9),
        1
    );

    // bob goes top left
    assert_eq!(
        client.play(&bob, &0),
        0
    );

    // alice goes top center
    assert_eq!(
        client.play(&alice, &1),
        0
    );


    // bob goes center left
    assert_eq!(
        client.play(&bob, &3),
        0
    );

    // alice goes bottom center (winning the game)
    assert_eq!(
        client.play(&alice, &7),
        0
    );

    // alice is the winner
    assert_eq!(
        client.winner(),
        alice.to_string()        
    );

    // bob goes center right should fail since game is over
    assert_eq!(
        client.play(&bob, &5),
        1
    );

    // bob start's a new game
    assert_eq!(
        client.start(&bob, &alice),
        0
    );
}
