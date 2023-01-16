use casper_dao_contracts::action::Action;
use casper_dao_utils::{DocumentHash, Error, TestContract};
use casper_types::{bytesrepr::Bytes, U512};

use crate::{
    common::{
        params::{
            voting::{Ballot, Voting, VotingType},
            Account,
            Balance,
            Contract,
        },
        DaoWorld,
    },
    on_voting_contract,
};

mod builder;

#[allow(dead_code)]
impl DaoWorld {
    pub fn checked_create_voting(&mut self, creator: Account, voting: Voting) -> Result<(), Error> {
        let creator = self.get_address(&creator);
        let stake = voting.get_stake();

        match builder::build(self, voting) {
            builder::VotingSetup::Admin(contract_to_update, action, subject) => self
                .admin
                .as_account(creator)
                .create_voting(contract_to_update, action, subject, *stake),
            builder::VotingSetup::Kyc(subject, document_hash) => self
                .kyc_voter
                .as_account(creator)
                .create_voting(subject, document_hash, *stake),
            builder::VotingSetup::Slasher(address_to_slash, slash_ratio) => self
                .slashing_voter
                .as_account(creator)
                .create_voting(address_to_slash, slash_ratio, *stake),
            builder::VotingSetup::Repository(
                variable_repository_address,
                key,
                value,
                activation_time,
            ) => self.repo_voter.as_account(creator).create_voting(
                variable_repository_address,
                key,
                value,
                activation_time,
                *stake,
            ),
            builder::VotingSetup::Simple(document_hash) => self
                .simple_voter
                .as_account(creator)
                .create_voting(document_hash, *stake),
            builder::VotingSetup::Reputation(recipient_address, action, amount, document_hash) => {
                self.reputation_voter.as_account(creator).create_voting(
                    recipient_address,
                    action,
                    *amount,
                    document_hash,
                    *stake,
                )
            }
        }
    }

    pub fn create_voting(&mut self, creator: Account, voting: Voting) {
        let contract = voting.contract;
        self.checked_create_voting(creator, voting)
            .unwrap_or_else(|_| panic!("Couldn't create {:?} voting", contract));
    }

    pub fn create_test_voting(&mut self, contract: Contract, creator: Account, stake: Balance) {
        let alice = self.get_address(&Account::Alice);
        let creator = self.get_address(&creator);
        let document_hash = Bytes::from(vec![1u8]);
        match contract {
            Contract::KycVoter => {
                self.kyc_voter
                    .as_account(creator)
                    .create_voting(alice, Bytes::new(), *stake)
            }
            Contract::RepoVoter => self.repo_voter.as_account(creator).create_voting(
                self.variable_repository.address(),
                String::from("key"),
                document_hash,
                None,
                *stake,
            ),
            Contract::ReputationVoter => self.reputation_voter.as_account(creator).create_voting(
                alice,
                casper_dao_contracts::reputation_voter::Action::Mint,
                U512::from(10),
                document_hash,
                *stake,
            ),
            Contract::Admin => self.admin.as_account(creator).create_voting(
                alice,
                casper_dao_contracts::action::Action::AddToWhitelist,
                alice,
                *stake,
            ),
            Contract::SlashingVoter => self
                .slashing_voter
                .as_account(creator)
                .create_voting(alice, 100, *stake),
            Contract::SimpleVoter => self
                .simple_voter
                .as_account(creator)
                .create_voting(document_hash, *stake),

            contract => panic!("{:?} is not a voting contract", contract),
        }
        .expect("Can't create voting")
    }

    pub fn vote(&mut self, contract: &Contract, ballot: &Ballot) {
        self.checked_vote(contract, ballot)
            .unwrap_or_else(|e| panic!("{:?} voting error: {e:?}", contract));
    }

    pub fn checked_vote(&mut self, contract: &Contract, ballot: &Ballot) -> Result<(), Error> {
        let voter = self.get_address(&ballot.voter);
        let voting_id = ballot.voting_id;
        let choice = ballot.choice.clone().into();
        let stake = ballot.stake.0;
        let voting_type = ballot.voting_type.into();

        on_voting_contract!(
            self,
            voter,
            contract,
            vote(voting_id, voting_type, choice, stake)
        )
    }

    pub fn finish_voting(
        &mut self,
        contract: &Contract,
        voting_id: u32,
        voting_type: Option<VotingType>,
    ) {
        let voting_type = voting_type.map(|vt| vt.into()).unwrap();

        match contract {
            Contract::KycVoter => self
                .kyc_voter
                .finish_voting(voting_id, voting_type)
                .expect("Couldn't finish KycVoter voting"),
            Contract::BidEscrow => self
                .bid_escrow
                .finish_voting(voting_id, voting_type)
                .expect("Couldn't finish BidEscrow voting"),
            Contract::SlashingVoter => self
                .slashing_voter
                .finish_voting(voting_id, voting_type)
                .expect("Couldn't finish SlashingVoting voting"),
            Contract::RepoVoter => self
                .repo_voter
                .finish_voting(voting_id, voting_type)
                .expect("Couldn't finish RepoVoter voting"),
            Contract::Admin => self
                .admin
                .finish_voting(voting_id, voting_type)
                .expect("Couldn't finish Admin voting"),
            Contract::SimpleVoter => self
                .simple_voter
                .finish_voting(voting_id, voting_type)
                .expect("Couldn't finish SimpleVoter voting"),
            Contract::ReputationVoter => self
                .reputation_voter
                .finish_voting(voting_id, voting_type)
                .expect("Couldn't finish ReputationVoter voting"),
            Contract::Onboarding => self
                .onboarding
                .finish_voting(voting_id, voting_type)
                .expect("Couldn't finish Onboarding voting"),
            invalid => panic!("{:?} is not a voting contract", invalid),
        };
    }

    pub fn voting_exists(
        &self,
        contract: &Contract,
        voting_id: u32,
        voting_type: VotingType,
    ) -> bool {
        let voting_type = voting_type.into();
        on_voting_contract!(self, contract, voting_exists(voting_id, voting_type))
    }

    pub fn checked_slash_voter(&mut self, contract: Contract, voter: Account, voting_id: u32) {
        let voter = self.get_address(&voter);
        let result = on_voting_contract!(self, contract, slash_voter(voter, voting_id));
        result.expect(&format!("Couldn't slash voter in {:?}", contract));
    }
}
