use casper_dao_utils::{BlockTime, DocumentHash, TestContract};
use casper_types::U512;

mod common;

// pub fn set_cspr_balance(&mut self, account: Address, amount: U512) {
//     assert!(!self.balances.contains_key(&account), "Cannot set cspr balance twice");
//
//     self.balances.insert(account, amount);
//
//     self.starting_balances
//         .insert(account, self.test_env().get_address_cspr_balance(account));
// }
//
// // gets relative amount of motes to the account
// pub fn get_cspr_balance(&self, account: Address) -> U512 {
//     self.balances.get(&account).unwrap() + self.test_env().get_address_cspr_balance(account)
//         - self.starting_balances.get(&account).unwrap()
// }

#[test]
fn test_balance() {
    let (env, mut bid_escrow, reputation_token, mut va_token, mut kyc_token, variable_repo) =
        common::dao::setup_dao();
    let bid_escrow_address = bid_escrow.address();
    assert_eq!(
        bid_escrow
            .get_env()
            .get_address_cspr_balance(bid_escrow_address),
        U512::zero()
    );
    let job_poster = bid_escrow.get_env().get_account(0);
    let worker = bid_escrow.get_env().get_account(1);
    kyc_token.mint(job_poster, 1.into()).unwrap();
    kyc_token.mint(worker, 2.into()).unwrap();
    va_token.mint(worker, 1.into()).unwrap();

    let job_poster_balance: U512 = bid_escrow.get_env().get_address_cspr_balance(job_poster);

    bid_escrow.pick_bid_with_cspr_amount(
        worker,
        DocumentHash::from(b"some hash".to_vec()),
        60,
        Some(500.into()),
        500.into(),
    );

    assert_eq!(
        bid_escrow
            .get_env()
            .get_address_cspr_balance(bid_escrow_address),
        500.into()
    );
    assert_eq!(
        bid_escrow.get_env().get_address_cspr_balance(job_poster),
        job_poster_balance - U512::from(500)
    );
}
