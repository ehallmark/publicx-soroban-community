#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger}, // AuthorizedFunction, AuthorizedInvocation},
    token, Address, Env
};

use token::Client as TokenClient;
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (Address, TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (   
        sac.address().clone(),
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

fn create_escrow_contract<'a>(e: &Env, admin: &Address) -> EscrowContractClient<'a> {
    EscrowContractClient::new(e, &e.register(EscrowContract, (admin, )))
}

struct EscrowTest<'a> {
    env: Env,
    depositor: Address,
    recipients: Vec<Address>,
    token: Address,
    token_client: TokenClient<'a>,
    contract_client: EscrowContractClient<'a>,
}

impl<'a> EscrowTest<'a> {
    fn setup() -> Self {
        let env = Env::default();
        env.mock_all_auths();

        env.ledger().with_mut(|li| {
            li.timestamp = 12345;
        });

        let depositor = Address::generate(&env);
        let token_admin = Address::generate(&env);
        let contract_admin = Address::generate(&env);

        let recipients = vec![&env,
            Address::generate(&env),
            Address::generate(&env),
            Address::generate(&env),
        ];

        
        let (token, token_client, token_admin_client) = create_token_contract(&env, &token_admin);
        token_admin_client.mint(&depositor, &1000);


        let contract_client = create_escrow_contract(&env, &contract_admin);
        EscrowTest {
            env,
            depositor,
            recipients,
            token,
            token_client,
            contract_client,
        }
    }
}


#[test]
fn test() {
    let test = EscrowTest::setup();
    let EscrowTest { env, depositor, recipients, token, token_client, contract_client } = test;
    assert_eq!(
        contract_client.deposit(
            &depositor,
            &recipients.get(0).unwrap(),
            &token,
            &100i128,
            &TimeBound {
                kind: TimeBoundKind::After,
                timestamp: 12344,
            },
        ), 
        (ReceiptConfig {
            amount: 100i128,
            token: token.clone(),
            depositor: depositor.clone(),
            time_bound: TimeBound {
                kind: TimeBoundKind::After,
                timestamp: 12344,
            }
        }, 0u32)
    );
    assert_eq!(
        contract_client.withdraw(
            &recipients.get(0).unwrap(),
            &1u32,
            &None
        ), (ReceiptConfig {
                amount: 100i128,
                token: token.clone(),
                depositor: depositor.clone(),
                time_bound: TimeBound {
                    kind: TimeBoundKind::After,
                    timestamp: 12344,
                }
            }, 0)
    );

}
