Feature: Kyc Voter errors
    VAs voting to pass the KYC process by Alice and Bob 
    Background:
      Given users
        | user    | is_va        | REP balance |
        | Alice   | false        | 0           |
        | VA1     | true         | 1000        |
        | VA2     | true         | 1000        |
        | VA3     | true         | 1000        |
        | VA4     | true         | 1000        |
        | VA5     | true         | 1000        |
        | VA6     | true         | 1000        |
      And VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |

    Scenario: Invalid votes
      When voters vote in KycVoter informal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 200        | in favor |
      Then votes in KycVoter informal voting with id 0 fail
        | user    | REP stake  | choice   | result              |
        | VA1     | 100        | against  | CannotVoteTwice     |
        | VA3     | 0          | against  | ZeroStake           |
        | VA4     | 1001       | against  | InsufficientBalance |
      And users balances are
        | account | REP balance  | REP stake  |
        | VA1     | 1000         | 100        |
        | VA2     | 1000         | 200        |
    
    Scenario: Informal voting quorum not reached
      When voters vote in KycVoter informal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 100        | in favor |
      And 5 days passed
      And informal voting with id 0 ends in KycVoter contract
      Then formal voting with id 0 in KycVoter contract does not start
      And users balances are
        | account | REP balance  | REP stake  |
        | VA1     | 1000         | 0          |
        | VA2     | 1000         | 0          |
        | VA3     | 1000         | 0          |
        | VA4     | 1000         | 0          |
        | VA5     | 1000         | 0          |
        | VA6     | 1000         | 0          |
    
    Scenario: Voting creation fails if there is ongoing voting already
      When VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |
      Then informal voting with id 1 in KycVoter contract does not start