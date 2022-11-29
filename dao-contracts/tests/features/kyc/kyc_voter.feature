Feature: Kyc Voter
    VAs voting to pass the KYC process by Alice and Bob 
    Background:
      Given users
        | user    | whitelisted_in | is_kyced | is_va        | REP balance |
        | Alice   | KycToken       | false    | false        | 0           |
        | Bob     | KycToken       | true     | false        | 0           |
        | VA1     | KycToken       | true     | true         | 1000        |
        | VA2     | KycToken       | true     | true         | 1000        |
        | VA3     | KycToken       | true     | true         | 1000        |
        | VA4     | KycToken       | true     | true         | 1000        |
        | VA5     | KycToken       | true     | true         | 1000        |
        | VA6     | KycToken       | true     | true         | 1000        |
      And Owner added KycVoter to whitelist in ReputationToken contract
      And Owner added VA1 to whitelist in KycVoter contract
      And VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |
    
    Scenario: Informal voting quorum not reached
      When voters vote in KycVoter's informal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 100        | in favor |
        | VA3     | 0          |          |
        | VA4     | 0          |          |
        | VA5     | 0          |          |
        | VA6     | 0          |          |
      And informal voting with id 0 ends in KycVoter contract
      Then formal voting with id 0 in KycVoter contract does not start
      And users balances are
        | account | REP balance  | REP stake  |
        | VA1     | 1000         | 0          |
        | VA2     | 1000         | 0          |
        | VA3     | 1000         | 0          |
        | VA4     | 1000         | 0          |
        | VA5     | 1000         | 0          |