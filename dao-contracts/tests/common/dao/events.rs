use casper_dao_modules::events::{AddedToWhitelist, OwnerChanged, RemovedFromWhitelist};
use casper_dao_utils::TestContract;

use crate::common::{
    params::{
        events::{Event, NtfEvent},
        Contract,
    },
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
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
