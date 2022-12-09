use casper_dao_utils::{DocumentHash, Error, TestContract};
use casper_types::bytesrepr::Bytes;

use crate::common::{
    params::{
        voting::{Ballot, Voting, VotingType},
        Account,
        Balance,
        Contract,
    },
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn checked_create_voting(&mut self, creator: Account, voting: Voting) -> Result<(), Error> {
        let creator = self.get_address(&creator);
        let stake = voting.get_stake();

        match voting.contract {
            Contract::KycVoter => {
                let subject_address = voting.get_parsed_arg::<Account>(0);
                let subject_address = self.get_address(&subject_address);
                self.kyc_voter.as_account(creator).create_voting(
                    subject_address,
                    DocumentHash::default(),
                    *stake,
                )
            }
            Contract::SlashingVoter => {
                let address_to_slash = voting.get_parsed_arg::<Account>(0);
                let address_to_slash = self.get_address(&address_to_slash);
                let slash_ratio = voting.get_parsed_arg::<f32>(1);

                self.slashing_voter.as_account(creator).create_voting(
                    address_to_slash,
                    (slash_ratio * 1000.0) as u32,
                    stake.0,
                )
            }
            contract => panic!("{:?} is not a voting contract", contract),
        }
    }

    pub fn create_voting(&mut self, creator: Account, voting: Voting) {
        let contract = voting.contract;
        self.checked_create_voting(creator, voting)
            .unwrap_or_else(|_| panic!("Couldn't create {:?} voting", contract));
    }

    pub fn create_random_voting(&mut self, contract: Contract, creator: Account, stake: Balance) {
        let alice = self.get_address(&Account::Alice);
        let creator = self.get_address(&creator);
        match contract {
            Contract::KycVoter => {
                self.kyc_voter
                    .as_account(creator)
                    .create_voting(alice, Bytes::new(), *stake)
            }
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

        match contract {
            Contract::KycVoter => {
                self.kyc_voter
                    .as_account(voter)
                    .vote(voting_id, voting_type, choice, stake)
            }
            Contract::BidEscrow => self
                .bid_escrow
                .as_account(voter)
                .vote(voting_id, choice, stake),
            Contract::SlashingVoter => {
                self.slashing_voter
                    .as_account(voter)
                    .vote(voting_id, voting_type, choice, stake)
            }
            contract => panic!("{:?} is not a voting contract", contract),
        }
    }

    pub fn finish_voting(
        &mut self,
        contract: &Contract,
        voting_id: u32,
        voting_type: Option<VotingType>,
    ) {
        let voting_type = voting_type.map(|vt| vt.into());

        match contract {
            Contract::KycVoter => self
                .kyc_voter
                .finish_voting(voting_id, voting_type.unwrap())
                .expect("Couldn't finish KycVoter voting"),
            Contract::BidEscrow => self
                .bid_escrow
                .finish_voting(voting_id)
                .expect("Couldn't finish BidEscrow voting"),
            Contract::SlashingVoter => self
                .slashing_voter
                .finish_voting(voting_id, voting_type.unwrap())
                .expect("Couldn't finish SlashingVoting voting"),
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
        match contract {
            Contract::KycVoter => self.kyc_voter.voting_exists(voting_id, voting_type),
            Contract::BidEscrow => todo!(),
            Contract::SlashingVoter => todo!(),
            invalid => panic!("{:?} is not a voting contract", invalid),
        }
    }

    pub fn checked_slash_voter(&mut self, contract: Contract, voter: Account, voting_id: u32) {
        let voter = self.get_address(&voter);
        match contract {
            Contract::KycVoter => self
                .kyc_voter
                .slash_voter(voter, voting_id)
                .expect("Couldn't slash voter in KycVoter"),
            invalid => panic!("{:?} is not a voting contract", invalid),
        }
    }
}
