Feature: User who wants to become a VA without submitting a job

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | is_kyced | is_va |
      | Onboarding       | 0            | 0            | false    | false |
      | MultisigWallet   | 0            | 0            | false    | false |
      | Alice            | 1000         | 0            | false    | false  |
      | Bob              | 2000         | 0            | true     | false |
      | VA1              | 0            | 1000         | true     | true  |
      | VA2              | 0            | 1000         | true     | true  |
      | VA3              | 1000         | 1000         | true     | true  |
    And following configuration
      | key                                    | value |
      | TimeBetweenInformalAndFormalVoting     | 0     |
      | VotingStartAfterJobSubmission          | 0     |
    When Bob submits an onboarding request with the stake of 1000 CSPR
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  | 
      | Onboarding       | 1000         | 0            | 0          |
      | Bob              | 1000         | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |

  Scenario: VAs reject the request
    When voters vote in Onboarding informal voting with id 0
      | user    | REP stake  | choice | 
     #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
      | VA1     | 500        | no     |
      | VA2     | 500        | no     |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 1000         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | Bob              | 1000         | 0            | 0          |
      | VA1              | 0            | 1000         | 500        |
      | VA2              | 0            | 1000         | 500        |
    And Onboarding total unbounded stake for voting 0 is 100 tokens
    And Onboarding ballot for voting 0 for Bob has 100 unbounded tokens
    When 6 days passed
    And informal voting with id 0 ends in Onboarding contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 1000         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | Bob              | 1000         | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And Onboarding total unbounded stake for voting 0 is 100 tokens
    And Onboarding ballot for voting 0 for Bob has 100 unbounded tokens
    When voters vote in Onboarding formal voting with id 0
      | user    | REP stake  | choice | 
     #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
      | VA1     | 500        | no     |
      | VA2     | 250        | yes    |
    And 6 days passed
    And formal voting with id 0 ends in Onboarding contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 0            | 0            | 0          |
      | MultisigWallet   | 100          | 0            | 0          |
      | Bob              | 1900         | 0            | 0          |
      | VA1              | 0            | 1250         | 0          |
      | VA2              | 0            | 750          | 0          |
    And total reputation is 3000
    And Bob is not a VA

  Scenario: Cannot submit the second request while voting is not completed
    When Bob submits an onboarding request with the stake of 1000 CSPR
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 1000         | 0            | 0          |
      | Bob              | 1000         | 0            | 0          |

  Scenario: A not kyed user cannot submit a request
    When Alice submits an onboarding request with the stake of 1000 CSPR
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 1000         | 0            | 0          |
      | Alice            | 1000         | 0            | 0          |

  Scenario: A not kyed user cannot submit a request
    When VA3 submits an onboarding request with the stake of 1000 CSPR
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 1000         | 0            | 0          |
      | VA3              | 1000         | 1000         | 0          |