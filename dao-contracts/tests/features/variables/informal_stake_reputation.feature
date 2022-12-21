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
      | key                                    | value  |
      | TimeBetweenInformalAndFormalVoting     | 0      |
      | InformalStakeReputation                | false  |
      | VotingStartAfterJobSubmission          | 0      |

    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker

  Scenario: Job made by Internal Worker - check if reputation is not staked
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When InternalWorker submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 500       | Yes    |
      | VA2              | 500       | Yes    |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When voters vote in BidEscrow formal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 500       | Yes    |
      | VA2              | 500       | No     |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 500        |
      | VA2              | 0            | 1000         | 500        |
    When 6 days passed
    And formal voting with id 0 ends in BidEscrow contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 50           | 0            | 0          |
      | JobPoster        | 500          | 0            | 0          |
      | InternalWorker   | 165.20       | 1119.69      | 0          |
      | VA1              | 210.02       | 1423.48      | 0          |
      | VA2              | 74.77        | 506.82       | 0          |
      | BidEscrow        | 0            | 0            | 0          |
    And total reputation is 3050
