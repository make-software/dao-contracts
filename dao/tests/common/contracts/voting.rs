use dao::voting_contracts::SlashedVotings;
use dao::{
    utils::{types::DocumentHash, Error},
    voting::{
        ballot::{Ballot as DaoBallot, Choice},
        types::VotingId,
        voting_engine::voting_state_machine::{
            VotingStateMachine, VotingSummary, VotingType as DaoVotingType,
        },
    },
};
use odra::{
    test_env,
    types::{Address, Balance, Bytes},
};

use crate::common::{
    params::{
        voting::{Ballot, Voting, VotingType},
        Account, Contract, ReputationBalance,
    },
    DaoWorld,
};

mod builder;

#[odra::external_contract]
pub trait Voter {
    fn vote(
        &mut self,
        voting_id: VotingId,
        voting_type: DaoVotingType,
        choice: Choice,
        stake: Balance,
    );
    fn finish_voting(&mut self, voting_id: VotingId, voting_type: DaoVotingType) -> VotingSummary;
    fn slash_voter(&mut self, voter: Address) -> SlashedVotings;
    fn voting_exists(&self, voting_id: VotingId, voting_type: DaoVotingType) -> bool;
    fn get_voting(&self, voting_id: VotingId) -> Option<VotingStateMachine>;
    fn get_ballot(
        &self,
        voting_id: VotingId,
        voting_type: DaoVotingType,
        address: Address,
    ) -> Option<DaoBallot>;
    fn cancel_finished_voting(&mut self, voting_id: VotingId);
}

#[allow(dead_code)]
impl DaoWorld {
    pub fn create_voting(&mut self, creator: Account, voting: Voting) {
        let stake = voting.get_stake();

        self.set_caller(&creator);

        match builder::build(self, voting) {
            builder::VotingSetup::Admin(contract_to_update, action, subject) => self
                .admin
                .create_voting(contract_to_update, action, subject, *stake),
            builder::VotingSetup::Kyc(subject, document_hash) => {
                self.kyc_voter.create_voting(subject, document_hash, *stake)
            }
            builder::VotingSetup::Slasher(address_to_slash, slash_ratio) => self
                .slashing_voter
                .create_voting(address_to_slash, slash_ratio, *stake),
            builder::VotingSetup::Repository(
                variable_repository_address,
                key,
                value,
                activation_time,
            ) => self.repo_voter.create_voting(
                variable_repository_address,
                key,
                value,
                activation_time,
                *stake,
            ),
            builder::VotingSetup::Simple(document_hash) => {
                self.simple_voter.create_voting(document_hash, *stake)
            }
            builder::VotingSetup::Reputation(recipient_address, action, amount, document_hash) => {
                self.reputation_voter.create_voting(
                    recipient_address,
                    action,
                    *amount,
                    document_hash,
                    *stake,
                )
            }
        }
    }

    pub fn create_test_voting(
        &mut self,
        contract: Contract,
        creator: Account,
        stake: ReputationBalance,
    ) {
        let alice = self.get_address(&Account::Alice);
        let va2 = self.get_address(&Account::VA(1));
        let document_hash = DocumentHash::from("123");

        self.set_caller(&creator);
        match contract {
            Contract::KycVoter => self.kyc_voter.create_voting(alice, document_hash, *stake),
            Contract::RepoVoter => self.repo_voter.create_voting(
                *self.variable_repository.address(),
                String::from("key"),
                Bytes::from(vec![1u8]),
                None,
                *stake,
            ),
            Contract::ReputationVoter => self.reputation_voter.create_voting(
                alice,
                dao::voting_contracts::ReputationAction::Mint,
                Balance::from(10),
                document_hash,
                *stake,
            ),
            Contract::Admin => self.admin.create_voting(
                alice,
                dao::voting_contracts::AdminAction::AddToWhitelist,
                alice,
                *stake,
            ),
            Contract::SlashingVoter => self.slashing_voter.create_voting(va2, 100, *stake),
            Contract::SimpleVoter => self.simple_voter.create_voting(document_hash, *stake),
            contract => panic!("{:?} is not a voting contract", contract),
        }
    }

    pub fn vote(&mut self, contract: &Account, ballot: &Ballot) {
        let voting_id = ballot.voting_id;
        let choice = ballot.choice.into();
        let stake = ballot.stake.0;
        let voting_type = ballot.voting_type.into();

        self.set_caller(&ballot.voter);
        let contract = self.get_address(contract);
        VoterRef::at(&contract).vote(voting_id, voting_type, choice, stake);
    }

    pub fn failing_vote(&mut self, contract: &Account, ballot: &Ballot, expected_error: Error) {
        let voting_id = ballot.voting_id;
        let choice = ballot.choice.into();
        let stake = ballot.stake.0;
        let voting_type = ballot.voting_type.into();

        self.set_caller(&ballot.voter);
        let contract = self.get_address(contract);
        test_env::assert_exception(expected_error, || {
            VoterRef::at(&contract).vote(voting_id, voting_type, choice, stake)
        })
    }

    pub fn finish_voting(
        &mut self,
        contract: &Account,
        voting_id: u32,
        voting_type: Option<VotingType>,
    ) {
        let voting_type = voting_type.map(|vt| vt.into()).unwrap();
        let contract = self.get_address(contract);
        VoterRef::at(&contract).finish_voting(voting_id, voting_type);
    }

    pub fn cancel_finished_voting(
        &mut self,
        contract: &Account,
        account: &Account,
        voting_id: u32,
    ) {
        let account = self.get_address(account);
        let contract = self.get_address(contract);
        test_env::set_caller(account);
        VoterRef::at(&contract).cancel_finished_voting(voting_id);
    }

    pub fn voting_exists(
        &self,
        contract: &Account,
        voting_id: u32,
        voting_type: VotingType,
    ) -> bool {
        let voting_type = voting_type.into();

        let contract = self.get_address(contract);
        VoterRef::at(&contract).voting_exists(voting_id, voting_type)
    }

    pub fn slash_voter(&mut self, caller: Account, contract: Account, voter: Account) {
        let caller = self.get_address(&caller);
        let voter = self.get_address(&voter);
        let contract = self.get_address(&contract);
        test_env::set_caller(caller);
        let _ = VoterRef::at(&contract).slash_voter(voter);
    }

    pub fn get_voting(&mut self, contract: &Account, voting_id: VotingId) -> VotingStateMachine {
        let voter = VoterRef::at(&self.get_address(contract));
        voter.get_voting(voting_id).expect("Voting does not exists")
    }

    pub fn get_ballot(
        &self,
        contract: &Account,
        account: &Account,
        voting_id: VotingId,
        voting_type: DaoVotingType,
    ) -> Option<DaoBallot> {
        let account = self.get_address(account);
        VoterRef::at(&self.get_address(contract)).get_ballot(voting_id, voting_type, account)
    }
}
