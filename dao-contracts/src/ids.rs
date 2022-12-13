use casper_dao_modules::AccessControl;
use casper_dao_utils::{
    casper_dao_macros::{casper_contract_interface, Instance},
    casper_env::caller,
    SequenceGenerator,
};

use crate::voting::VotingId;

#[casper_contract_interface]
pub trait DaoIdsContractInterface {
    ///  Initializes a contract.
    ///  Sets the deployer as the owner.
    ///
    ///  see [AccessControl](AccessControl)
    fn init(&mut self);
    /// Returns the next voting id in the system.
    ///
    /// # Errors
    /// Throws [`NotWhitelisted`](casper_dao_utils::Error::NotWhitelisted) if the caller is not whitelisted.
    fn next_voting_id(&mut self) -> VotingId;
}

#[derive(Instance)]
pub struct DaoIdsContract {
    access_control: AccessControl,
    voting_id_seq: SequenceGenerator<VotingId>,
}

impl DaoIdsContractInterface for DaoIdsContract {
    fn init(&mut self) {
        let deployer = caller();
        self.access_control.init(deployer);
    }

    fn next_voting_id(&mut self) -> VotingId {
        self.access_control.ensure_whitelisted();
        self.voting_id_seq.next_value()
    }
}
