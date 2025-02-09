#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Address, Symbol, String};

#[contract]
pub struct Contract;

const X: Symbol = symbol_short!("X");
const O: Symbol = symbol_short!("O");
const NULL: Symbol = symbol_short!("NULL");

const PLAYER: Symbol = symbol_short!("PLAYER");
const OPPONENT: Symbol = symbol_short!("OPPONENT");
const PLAYING: Symbol = symbol_short!("PLAYING");
const NEXT: Symbol = symbol_short!("NEXT");
const WINNER: Symbol = symbol_short!("WINNER");

#[contractimpl]
impl Contract {
    pub fn start(env: Env, player_addr: Address, opponent_addr: Address) -> u32 {
        player_addr.require_auth();

        let mut playing: u32 = env.storage().instance().get(&PLAYING).unwrap_or(0);
        log!(&env, "playing", &playing);
        if playing == 0 {
            log!(&env, "not playing");
            let empty: String = String::from_str(&env, "");
            env.storage().instance().set(&PLAYER, &player_addr.to_string());
            env.storage().instance().set(&OPPONENT, &opponent_addr.to_string());
            env.storage().instance().set(&NEXT, &player_addr.to_string());
            env.storage().instance().set(&WINNER, &empty);
            for i in 0..9u32 {
                env.storage().instance().set(&i, &NULL);
            }
            // update state to "playing"
            playing += 1;            
            env.storage().instance().set(&PLAYING, &playing);
            env.storage().instance().extend_ttl(100, 100);
            0
        } else {               
            log!(&env, "playing");
            1
        }
    }

    pub fn play(env: Env, addr: Address, player_move: u32) -> u32 {
        log!(&env, "play...");
        addr.require_auth();
        
        log!(&env, "pre assert...");

        let playing: u32 = env.storage().instance().get(&PLAYING).unwrap_or(0);

        if playing == 0 {
            log!(&env, "not playing!");
            return 1;
        }

        log!(&env, "post assert...");
        
        let empty: String = String::from_str(&env, "");

        // make sure player is next
        let mut next: String = env.storage().instance().get(&NEXT).unwrap_or(empty.clone());
        if next != addr.to_string() {
            // bad
            log!(&env, "user is not next to play");
            return 1;
        }

        // check if player is playing
        let value: Symbol;
        let player: String = env.storage().instance().get(&PLAYER).unwrap_or(empty.clone());
        let opponent: String = env.storage().instance().get(&OPPONENT).unwrap_or(empty.clone());


        if player == addr.to_string() {
            log!(&env, "is player");
            next = opponent;
            value = X;
        } else if opponent == addr.to_string() {
            log!(&env, "is opponent");
            next = player;
            value = O;
        } else {
            // bad case!
            log!(&env, "BAD!");
            return 1;
        }

        log!(&env, "player move: {}", &player_move);

        if player_move < 9 {
            // valid move
            if env.storage().instance().get(&player_move).unwrap_or(NULL) != NULL {
                log!(&env, "someone already played this tile");
                return 1;
            }
        } else {
            return 1;
        }

        env.storage().instance().set(&player_move, &value);
        env.storage().instance().set(&NEXT, &next);
        env.storage().instance().extend_ttl(100, 100);

        // check rows and cols
        let mut win: bool = false;
        for i in 0u32..3u32 {
            if env.storage().instance().get(&(i*3u32)).unwrap_or(NULL) == value && env.storage().instance().get(&(i*3 + 1)).unwrap_or(NULL) == value && env.storage().instance().get(&(i*3 + 2)).unwrap_or(NULL) == value {
                win = true;
                break;
            }
            if env.storage().instance().get(&(i)).unwrap_or(NULL) == value && env.storage().instance().get(&(1*3u32 + i)).unwrap_or(NULL) == value && env.storage().instance().get(&(2*3u32 + i)).unwrap_or(NULL) == value {
                win = true;
                break;
            }
        }

        // check diagonals
        if !win {
            if env.storage().instance().get(&0u32).unwrap_or(NULL) == value && env.storage().instance().get(&4u32).unwrap_or(NULL) == value && env.storage().instance().get(&8u32).unwrap_or(NULL) == value {
                win = true;
            }
            if env.storage().instance().get(&2u32).unwrap_or(NULL) == value && env.storage().instance().get(&4u32).unwrap_or(NULL) == value && env.storage().instance().get(&6u32).unwrap_or(NULL) == value {
                win = true;
            }
        }

        if win {
            env.storage().instance().set(&WINNER, &addr.to_string());
            env.storage().instance().set(&PLAYING, &0u32);        
        }

        log!(&env, "Board:");
        log!(&env, "", env.storage().instance().get(&0u32).unwrap_or(NULL),
                               env.storage().instance().get(&1u32).unwrap_or(NULL),
                               env.storage().instance().get(&2u32).unwrap_or(NULL));
        log!(&env, "", env.storage().instance().get(&3u32).unwrap_or(NULL),
                               env.storage().instance().get(&4u32).unwrap_or(NULL),
                               env.storage().instance().get(&5u32).unwrap_or(NULL));
        log!(&env, "", env.storage().instance().get(&6u32).unwrap_or(NULL),
                               env.storage().instance().get(&7u32).unwrap_or(NULL),
                               env.storage().instance().get(&8u32).unwrap_or(NULL));
        log!(&env, "------------");
        0
    }

    pub fn winner(env: Env) -> String {
        let empty: String = String::from_str(&env, "");
        log!(&env, "", env.storage().instance().get(&WINNER).unwrap_or(empty.clone()));
        env.storage().instance().get(&WINNER).unwrap_or(empty)
    }
}

mod test;
