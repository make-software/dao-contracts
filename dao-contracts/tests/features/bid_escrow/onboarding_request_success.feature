Feature: User who wants to become a VA without submitting a job

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | is_kyced | is_va |
      | Onboarding        | 0            | 0            | false    | false |
      | MultisigWallet   | 0            | 0            | false    | false |
      | Bob              | 1000         | 0            | true     | false |
      | VA1              | 0            | 1000         | true     | true  |
      | VA2              | 0            | 1000         | true     | true  |
    And following configuration
      | key                                    | value |
      | TimeBetweenInformalAndFormalVoting     | 0     |
      | VotingStartAfterJobSubmission          | 0     |
    When Bob submits an onboarding request with the stake of 1000 CSPR
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  | 
      | Onboarding        | 1000         | 0            | 0          |
      | Bob              | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |

  Scenario: VAs votes to accept the request
    When voters vote in Onboarding informal voting with id 0
      | user    | REP stake  | choice | 
     #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
      | VA1     | 500        | yes    |
      | VA2     | 500        | yes    |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 1000         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | Bob              | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 500        |
      | VA2              | 0            | 1000         | 500        |
    And total onboarding unbounded stake for voting 0 is 100 tokens
    And ballot for voting 0 for Bob has 100 unbounded tokens
    When Informal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | Onboarding       | 1000         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | Bob              | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And total onboarding unbounded stake for voting 0 is 100 tokens
    And ballot for voting 0 for Bob has 100 unbounded tokens
    When voters vote in Onboarding formal voting with id 0
      | user    | REP stake  | choice | 
     #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
      | VA1     | 500        | yes    |
      | VA2     | 500        | yes    |
    And Formal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 100          | 0            | 0          |
      | Bob              | 31.16        | 72.72        | 0          |
      | VA1              | 434.41       | 1013.63      | 0          |
      | VA2              | 434.41       | 1013.63      | 0          |
      | BidEscrow        | 0            | 0            | 0          |
    And total reputation is 2100
    And Bob is a VA



      # | MultisigWallet   | 100          | 0            | 0          |
      # | Bob              | 30           | 70           | 0          |
      # | VA1              | 435          | 1015         | 0          |
      # | VA2              | 435          | 1015         | 0          |
      # | BidEscrow        | 0            | 0            | 0          |