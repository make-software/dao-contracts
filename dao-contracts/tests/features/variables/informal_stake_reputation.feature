Feature: InformalStakeReputation variable
  If set to false, the reputation will not be staked during informal voting.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | MultisigWallet   | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
      | InformalStakeReputation                | false         |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker

  Scenario: Job made by Internal Worker - check if reputation is not staked
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When InternalWorker submits the JobProof
    And votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | Yes  | 500   |
      | VA2              | Yes  | 500   |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When Informal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | Yes  | 500   |
      | VA2              | No   | 500   |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 500        |
      | VA2              | 0            | 1000         | 500        |
    When Formal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 50           | 0            | 0          |
      | JobPoster        | 500          | 0            | 0          |
      | InternalWorker   | 165.20       | 1119.69      | 0          |
      | VA1              | 210.02       | 1423.48      | 0          |
      | VA2              | 74.77        | 506.82       | 0          |
      | BidEscrow        | 0            | 0            | 0          |
    And total reputation is 3050
