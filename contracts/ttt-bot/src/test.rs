#![cfg(test)]

use crate::{tictactoe, BotContract, BotContractClient};

use super::*;
use soroban_sdk::{vec, Env, String, testutils::Address as _};

#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(BotContract, ());
    let game_contract_id: Address = env.register(tictactoe::WASM, ());
    let game_client = tictactoe::Client::new(&env, &game_contract_id);
    let client = BotContractClient::new(&env, &contract_id);
    
    let alice = Address::generate(&env);

    // ai can't make a move before game is started
    assert_eq!(
        client.go(&game_contract_id),
        1
    );

    // alice start's the game
    assert_eq!(
        game_client.start(&alice, &contract_id),
        0
    );

    // alice goes center center
    assert_eq!(
        game_client.play(&alice, &4u32),
        0
    );

    // ai makes a move
    assert_eq!(
        client.go(&game_contract_id),
        0
    );

    // ai can't make 2 moves in a row
    assert_eq!(
        client.go(&game_contract_id),
        1
    );

    // alice goes top center
    assert_eq!(
        game_client.play(&alice, &1u32),
        0
    );

    // ai makes a move
    assert_eq!(
        client.go(&game_contract_id),
        0
    );

    // alice goes bottom center
    assert_eq!(
        game_client.play(&alice, &7u32),
        0
    );

    // alice is the winner
    assert_eq!(
        game_client.winner(),
        alice.to_string()        
    );

    // ai can't make a move after game is over
    assert_eq!(
        client.go(&game_contract_id),
        1
    );

}
