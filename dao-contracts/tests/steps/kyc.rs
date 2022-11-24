use casper_dao_utils::TestContract;
use cucumber::{given, then, when};

use crate::common::{DaoWorld, params::{
        common::U256,
        nft::{Account, TokenId},
    }};

#[given(expr = "{account} that owns a KYC Token")]
fn setup_user_with_token(world: &mut DaoWorld, user: Account) {
    world.checked_mint(&Account::Owner, &user);

    assert_eq!(world.balance_of(&user), U256::one());
}

#[when(expr = "{account} mints a KYC Token to {account}")]
fn mint(world: &mut DaoWorld, minter: Account, recipient: Account) {
    let _ = world.mint(&minter, &recipient);
}

#[when(expr = "{account} burns {account}'s token")]
fn burn(world: &mut DaoWorld, burner: Account, holder: Account) {
    let _ = world.burn(&burner, &holder);
}

#[then(expr = "the {account}'s balance is {u256}")]
fn assert_balance(world: &mut DaoWorld, user: Account, expected_balance: U256) {
    assert_eq!(world.balance_of(&user), expected_balance);
}

#[then(expr = "Token with id {token_id} belongs to {account}")]
fn assert_token_ownership(world: &mut DaoWorld, token_id: TokenId, user: Account) {
    let token_owner = world.kyc_token.owner_of(*token_id);
    let user_address = user.get_address(world);

    assert_eq!(token_owner, Some(user_address));
    assert_eq!(world.token_id(&user), U256::one());
}

#[then(expr = "total supply is {u256} token(s)")]
fn assert_total_supply(world: &mut DaoWorld, expected_total_supply: U256) {
    let total_supply = world.kyc_token.total_supply();
    assert_eq!(total_supply, expected_total_supply.0);
}

impl DaoWorld {

    fn mint(&mut self, minter: &Account, recipient: &Account) -> Result<(), casper_dao_utils::Error> {
        let minter = minter.get_address(self);
        let recipient = recipient.get_address(self);

        self
            .kyc_token
            .as_account(minter)
            .mint(recipient)
    }

    fn checked_mint(&mut self, minter: &Account, recipient: &Account) {
        self.mint(minter, recipient).expect("A token should be minted");
    }
    
    fn burn(&mut self, burner: &Account, holder: &Account) -> Result<(), casper_dao_utils::Error> {
        let token_id = self.token_id(holder);
        let burner = burner.get_address(self);
        
        self
        .kyc_token
        .as_account(burner)
        .burn(*token_id)
    }

    fn checked_burn(&mut self, minter: &Account, recipient: &Account) {
        self.burn(minter, recipient).expect("A token should be burned");
    }

    fn token_id(&self, holder: &Account) -> U256 {
        let holder = holder.get_address(self);
        let id = self.kyc_token.token_id(holder).expect("Holder should own a token");
        U256(id)
    } 

    fn balance_of(&self, account: &Account) -> U256 {
        U256(self.kyc_token.balance_of(account.get_address(self)))
    }
}
