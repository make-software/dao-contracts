Feature: Internal Flow
  External and Internal Workers are submitting a bid.
  Job Poster picks a bid of an Internal Worker, and the Internal Worker accepts the job.
  External Worker's stake is returned.
  The voting process is completed.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | MultisigWallet   | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | ExternalWorker   | 500          | 0            | 0          | true     | false |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
      | VotingStartAfterJobSubmission          | 0             |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 CSPR stake with onboarding
    And JobPoster picked the Bid of InternalWorker

  Scenario: JobPoster picked the Bid of Internal Worker
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | ExternalWorker   | 500          | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When InternalWorker submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 500       | Yes    |
      | VA2              | 500       | Yes    |
    And 6 days passed
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
    And 6 days passed
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
