Feature: User who wants to become a VA without submitting a job

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | false    | false |
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
      | BidEscrow        | 1000         | 0            | 0          |
      | Bob              | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |

  Scenario: VAs votes to accept the request
    When votes are
      | account          | vote | stake |
     #| Bob              | Yes  | 100   | - automatically voted by the system - 1000CSPR converted to 100 Reputation
      | VA1              | No   | 500   |
      | VA2              | No   | 500   |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1000         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | Bob              | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 500        |
      | VA2              | 0            | 1000         | 500        |
    And total unbounded stake for voting 0 is 100 tokens
    And ballot for voting 0 for Bob has 100 unbounded tokens
    When Informal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1000         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | Bob              | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And total unbounded stake for voting 0 is 100 tokens
    And ballot for voting 0 for Bob has 100 unbounded tokens
    When votes are
      | account          | vote | stake |
     #| Bob              | Yes  | 100   | - automatically voted by the system
      | VA1              | No   | 500   |
      | VA2              | No   | 500   |
    And Formal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 0            | 0            | 0          |
      | Bob              | 1000         | 0            | 0          |
      | VA1              | 400          | 1050         | 0          |
      | VA2              | 400          | 1050         | 0          |
      | BidEscrow        | 0            | 0            | 0          |
    And total reputation is 2100
    And Bob is a VA