#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, vec, Address, Env, Symbol, Vec};

mod tictactoe {
    soroban_sdk::contractimport!(
        file = "../../target/wasm32-unknown-unknown/release/tictactoe.wasm"
    );
}

const NULL: Symbol = symbol_short!("N");

#[contract]
pub struct BotContract;

#[contractimpl]
impl BotContract {
    pub fn go(env: Env, contract: Address) -> u32 {
        let client = tictactoe::Client::new(&env, &contract);

        log!(&env, "{}", env.current_contract_address());
        log!(&env, "is playing: ", client.is_playing());
        if client.is_playing() < 1 {
            log!(&env, "Game has not started yet.");
            return 1;
        }

        if client.winner().len() > 0 {
            log!(&env, "There is already a winner!");
            return 1;
        }

        let board: Vec<Symbol> = client.display();
        // check which indices are empty
        let mut moves: Vec<u32> = vec![&env];
        for i in 0..9u32 {
            if board.get(i) == Some(NULL) {
                moves.push_back(i);
            }
        }

        log!(&env, "available moves: ", moves);

        if moves.len() == 0 {
            log!(&env, "Game board is full...");
            return 1;
        }

        // make random move
        let mut value: u64 = 0;
        env.prng().fill(&mut value);
        let player_move: u32 = moves.get((value % (moves.len() as u64)).try_into().unwrap()).unwrap();
        log!(&env, "making move: ", player_move);
        let res: u32 = client.play(
            &env.current_contract_address(),
            &player_move
        );
        log!(&env, "result: {}", res);
        return res;
    }
}

mod test;
