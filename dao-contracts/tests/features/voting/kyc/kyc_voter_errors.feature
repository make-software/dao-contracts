Feature: Kyc Voter errors
    VAs voting to KYC by Alice

    Scenario: Voting creation fails if there is ongoing voting already
      Given users
        | user    | is_va        | REP balance |
        | Alice   | false        | 0           |
        | VA1     | true         | 1000        |
        | VA2     | true         | 1000        |
      When VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |
      When VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |
      Then informal voting with id 1 in KycVoter contract does not start
    
    Scenario: Cannot create a voting for a user who already is a va
      Given users
        | user    | is_va        | is_kyced | REP balance |
        | Alice   | false        | true     | 0           |
        | VA1     | true         | true     | 1000        |
        | VA2     | true         | true     | 1000        |
      When VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |
      Then informal voting with id 0 in KycVoter contract does not start
      