Feature: Kyc Voter
    VAs voting to pass the KYC process by Alice and Bob 
    Background:
      Given users in KycToken contract
        | user    | is_whitelisted | is_kyced | is_va        | REP balance |
        | Alice   | true           | false    | false        | 0           |
        | Bob     | true           | true     | false        | 0           |
        | VA1     | true           | true     | true         | 1000        |
        | VA2     | true           | true     | true         | 1000        |
        | VA3     | true           | true     | true         | 1000        |
        | VA4     | true           | true     | true         | 1000        |
        | VA5     | true           | true     | true         | 1000        |
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
      And Informal voting ends in KycVoter contract
      Then Formal voting does not start
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |