use casper_dao_utils::{DocumentHash, TestContract};

use crate::common::{
    params::{
        voting::{Ballot, Voting},
        Account,
        Contract,
    },
    DaoWorld,
};

#[allow(dead_code)]
impl DaoWorld {
    pub fn create_voting(&mut self, creator: Account, voting: Voting) {
        let creator = self.get_address(&creator);
        let stake = voting.get_stake();

        match voting.contract {
            Contract::KycVoter => {
                let subject_address = voting.get_parsed_arg::<Account>(0);
                let subject_address = self.get_address(&subject_address);

                self.kyc_voter
                    .as_account(creator)
                    .create_voting(subject_address, DocumentHash::default(), stake)
                    .expect("Couldn't create Kyc voting");
            }
            Contract::BidEscrow => todo!(),
            Contract::SlashingVoter => {
                let address_to_slash = voting.get_parsed_arg::<Account>(0);
                let address_to_slash = self.get_address(&address_to_slash);
                let slash_ratio = voting.get_parsed_arg::<u32>(1);

                self.slashing_voter
                    .as_account(creator)
                    .create_voting(address_to_slash, slash_ratio, stake)
                    .expect("Couldn't create Slashing voting");
            }
            contract => panic!("{:?} is not a voting contract", contract),
        }
    }

    pub fn vote(&mut self, contract: &Contract, ballot: Ballot) {
        let voter = self.get_address(&ballot.voter);
        let voting_id = ballot.voting_id;
        let choice = ballot.choice.into();
        let stake = ballot.stake.0;
        let voting_type = ballot.voting_type.into();

        if stake.is_zero() {
            return;
        }

        match contract {
            Contract::KycVoter => self
                .kyc_voter
                .as_account(voter)
                .vote(voting_id, voting_type, choice, stake)
                .expect("Kyc voting error"),
            Contract::BidEscrow => self
                .bid_escrow
                .as_account(voter)
                .vote(voting_id, choice, stake)
                .expect("Bid escrow voting error"),
            Contract::SlashingVoter => self
                .slashing_voter
                .as_account(voter)
                .vote(voting_id, voting_type, choice, stake)
                .expect("Slashing voting error"),
            contract => panic!("{:?} is not a voting contract", contract),
        }
    }
}
