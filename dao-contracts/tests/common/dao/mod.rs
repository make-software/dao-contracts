use casper_dao_contracts::{
    BidEscrowContractTest,
    KycNftContractTest,
    ReputationContractTest,
    SlashingVoterContractTest,
    VaNftContractTest,
    VariableRepositoryContractTest,
};
use casper_dao_modules::events::{AddedToWhitelist, OwnerChanged, RemovedFromWhitelist};
use casper_dao_utils::{Address, TestContract, TestEnv};

use super::params::common::{Event, NtfEvent};
use crate::common::{
    params::{common::Contract, nft::Account},
    DaoWorld,
};

impl DaoWorld {
    pub fn whitelist(
        &mut self,
        contract: &Contract,
        caller: &Account,
        user: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let user = user.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.as_account(caller).add_to_whitelist(user),
            Contract::VaToken => self.va_token.as_account(caller).add_to_whitelist(user),
            Contract::ReputationToken => self
                .reputation_token
                .as_account(caller)
                .add_to_whitelist(user),
            Contract::VariableRepository => {
                self.variable_repo.as_account(caller).add_to_whitelist(user)
            }
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn remove_from_whitelist(
        &mut self,
        contract: &Contract,
        caller: &Account,
        user: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let user = user.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self
                .kyc_token
                .as_account(caller)
                .remove_from_whitelist(user),
            Contract::VaToken => self.va_token.as_account(caller).remove_from_whitelist(user),
            Contract::ReputationToken => self
                .reputation_token
                .as_account(caller)
                .remove_from_whitelist(user),
            Contract::VariableRepository => self
                .variable_repo
                .as_account(caller)
                .remove_from_whitelist(user),
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn get_owner(&mut self, contract: &Contract) -> Option<Address> {
        match contract {
            Contract::KycToken => self.kyc_token.get_owner(),
            Contract::VaToken => self.va_token.get_owner(),
            Contract::ReputationToken => self.reputation_token.get_owner(),
            Contract::VariableRepository => self.variable_repo.get_owner(),
            _ => None,
        }
    }

    pub fn change_ownership(
        &mut self,
        contract: &Contract,
        caller: &Account,
        new_owner: &Account,
    ) -> Result<(), casper_dao_utils::Error> {
        let new_owner = new_owner.get_address(self);
        let caller = caller.get_address(self);

        match contract {
            Contract::KycToken => self
                .kyc_token
                .as_account(caller)
                .change_ownership(new_owner),
            Contract::VaToken => self.va_token.as_account(caller).change_ownership(new_owner),
            Contract::ReputationToken => self
                .reputation_token
                .as_account(caller)
                .change_ownership(new_owner),
            Contract::VariableRepository => self
                .variable_repo
                .as_account(caller)
                .change_ownership(new_owner),
            _ => Err(casper_dao_utils::Error::Unknown),
        }
    }

    pub fn is_whitelisted(&mut self, contract: &Contract, account: &Account) -> bool {
        let account = account.get_address(self);

        match contract {
            Contract::KycToken => self.kyc_token.is_whitelisted(account),
            Contract::VaToken => self.va_token.is_whitelisted(account),
            Contract::ReputationToken => self.reputation_token.is_whitelisted(account),
            Contract::VariableRepository => self.variable_repo.is_whitelisted(account),
            _ => false,
        }
    }

    pub fn assert_event(&self, contract: &Contract, idx: i32, ev: Event) {
        match ev {
            Event::OwnerChanged(account) => {
                let new_owner = account.get_address(self);
                self._assert_event(contract, idx, OwnerChanged { new_owner })
            }
            Event::AddedToWhitelist(account) => {
                let address = account.get_address(self);
                self._assert_event(contract, idx, AddedToWhitelist { address })
            }
            Event::RemovedFromWhitelist(account) => {
                let address = account.get_address(self);
                self._assert_event(contract, idx, RemovedFromWhitelist { address })
            }
            Event::NtfEvent(ntf_ev) => match ntf_ev {
                NtfEvent::Transfer(from, to, token_id) => {
                    let from = from.map(|account| account.get_address(self));
                    let to = to.map(|account| account.get_address(self));
                    let token_id = token_id.0;

                    self._assert_event(
                        contract,
                        idx,
                        casper_dao_erc721::events::Transfer { from, to, token_id },
                    )
                }
                NtfEvent::Approval(owner, approved, token_id) => {
                    let owner = owner.map(|account| account.get_address(self));
                    let approved = approved.map(|account| account.get_address(self));
                    let token_id = token_id.0;

                    self._assert_event(
                        contract,
                        idx,
                        casper_dao_erc721::events::Approval {
                            owner,
                            approved,
                            token_id,
                        },
                    )
                }
            },
        };
    }

    fn _assert_event<T>(&self, contract: &Contract, idx: i32, ev: T)
    where
        T: casper_types::bytesrepr::FromBytes + std::cmp::PartialEq + std::fmt::Debug,
    {
        match contract {
            Contract::KycToken => TestContract::assert_event_at(&self.kyc_token, idx, ev),
            Contract::VaToken => TestContract::assert_event_at(&self.va_token, idx, ev),
            Contract::ReputationToken => {
                TestContract::assert_event_at(&self.reputation_token, idx, ev)
            }
            Contract::VariableRepository => {
                TestContract::assert_event_at(&self.variable_repo, idx, ev)
            }
            Contract::BidEscrow => TestContract::assert_event_at(&self.bid_escrow, idx, ev),
            Contract::SlashingVoter => TestContract::assert_event_at(&self.slashing_voter, idx, ev),
        }
    }
}

#[allow(dead_code)]
pub fn setup_dao() -> (
    TestEnv,
    BidEscrowContractTest,
    ReputationContractTest,
    VaNftContractTest,
    KycNftContractTest,
    VariableRepositoryContractTest,
    SlashingVoterContractTest,
) {
    let env = TestEnv::new();
    let variable_repo = VariableRepositoryContractTest::new(&env);
    let mut reputation_token = ReputationContractTest::new(&env);

    let mut va_token = VaNftContractTest::new(
        &env,
        "va_token".to_string(),
        "VAT".to_string(),
        "".to_string(),
    );

    let kyc_token = KycNftContractTest::new(
        variable_repo.get_env(),
        "kyc token".to_string(),
        "kyt".to_string(),
        "".to_string(),
    );

    let bid_escrow = BidEscrowContractTest::new(
        variable_repo.get_env(),
        variable_repo.address(),
        reputation_token.address(),
        kyc_token.address(),
        va_token.address(),
    );

    let slashing_voter = SlashingVoterContractTest::new(
        variable_repo.get_env(),
        variable_repo.address(),
        reputation_token.address(),
        va_token.address(),
    );

    reputation_token
        .add_to_whitelist(bid_escrow.address())
        .unwrap();

    reputation_token
        .add_to_whitelist(slashing_voter.address())
        .unwrap();

    va_token.add_to_whitelist(bid_escrow.address()).unwrap();
    (
        env,
        bid_escrow,
        reputation_token,
        va_token,
        kyc_token,
        variable_repo,
        slashing_voter,
    )
}
