#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    testutils::Address as _, // AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env
};

use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}


#[test]
fn test() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let bob = Address::generate(&env);

    let (token_a, token_a_admin) = create_token_contract(&env, &token_admin);
    token_a_admin.mint(&alice, &1000);

    let contract_id = env.register(EscrowContract, (&contract_admin, ));
    let client = EscrowContractClient::new(&env, &contract_id);

    assert_eq!(
        client.deposit(
            &alice,
            &bob,
            &token_a.address,
            &100i128,
            &TimeBound {
                kind: TimeBoundKind::Before,
                timestamp: 12344,
            },
        ), 
        (ReceiptConfig {
            amount: 100i128,
            token: token_a.address.clone(),
            depositor: alice.clone(),
            time_bound: TimeBound {
                kind: TimeBoundKind::Before,
                timestamp: 12344,
            }
        }, 0u32)
    );

    assert_eq!(
        client.withdraw(
            &bob,
            &1u32,
        ), (ReceiptConfig {
                amount: 100i128,
                token: token_a.address.clone(),
                depositor: alice.clone(),
                time_bound: TimeBound {
                    kind: TimeBoundKind::Before,
                    timestamp: 12344,
                }
            }, 0)
    );

}
