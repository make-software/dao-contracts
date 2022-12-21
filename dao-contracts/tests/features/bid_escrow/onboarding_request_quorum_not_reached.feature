Feature: User who wants to become a VA without submitting a job

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | is_kyced | is_va |
      | Onboarding       | 0            | 0            | false    | false |
      | MultisigWallet   | 0            | 0            | false    | false |
      | Bob              | 1000         | 0            | true     | false |
      | VA1              | 0            | 1000         | true     | true  |
      | VA2              | 0            | 1000         | true     | true  |
      | VA3              | 0            | 1000         | true     | true  |
      | VA4              | 0            | 1000         | true     | true  |
      | VA5              | 0            | 1000         | true     | true  |
      | VA6              | 0            | 1000         | true     | true  |
    And following configuration
      | key                                    | value |
      | TimeBetweenInformalAndFormalVoting     | 0     |
      | VotingStartAfterJobSubmission          | 0     |
    When Bob submits an onboarding request with the stake of 1000 CSPR
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  | 
      | Onboarding       | 1000         | 0            | 0          |
      | Bob              | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
    And total reputation is 6000
  
  Scenario: Informal voting does not reach quorum
    When voters vote in Onboarding informal voting with id 0
      | user    | REP stake  | choice | 
     #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
      | VA1     | 500        | yes    |
    When 6 days passed
    And informal voting with id 0 ends in Onboarding contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | Bob              | 1000         | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
    And total reputation is 6000
    And Bob is not a VA
  Scenario: Formal voting does not reach quorum
    When voters vote in Onboarding informal voting with id 0
        | user    | REP stake  | choice | 
       #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
        | VA1     | 500        | yes    |
        | VA2     | 500        | yes    |
        | VA3     | 500        | yes    |
      When 6 days passed
      And informal voting with id 0 ends in Onboarding contract
      And voters vote in Onboarding formal voting with id 0
        | user    | REP stake  | choice | 
       #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
        | VA1     | 500        | yes    |
      And 6 days passed
      And formal voting with id 0 ends in Onboarding contract
      Then balances are
        | account          | CSPR balance | REP balance  | REP stake  |
        | Onboarding       | 0            | 0            | 0          |
        | MultisigWallet   | 0            | 0            | 0          |
        | Bob              | 1000         | 0            | 0          |
        | VA1              | 0            | 1000         | 0          |
        | VA2              | 0            | 1000         | 0          |
        | VA3              | 0            | 1000         | 0          |
      And total reputation is 6000
      And Bob is not a VA
