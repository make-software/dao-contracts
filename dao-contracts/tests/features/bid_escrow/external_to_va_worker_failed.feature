Feature: External Worker who wants to become a VA submits job
  Job Poster picks a bid of an External Worker, and the External Worker does the job.
  The External Worker wants to become a VA.
  The formal voting fails.

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
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding
    And JobPoster picked the Bid of ExternalWorker

  Scenario: JobPoster picked the Bid of External Worker
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1400         | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When ExternalWorker submits the JobProof
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| ExternalWorker   | 50        | Yes   | - automatically voted by the system - 500CSPR converted to 50 Reputation
      | VA1              | 500       | No    |
      | VA2              | 500       | No    |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1400         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 500        |
      | VA2              | 0            | 1000         | 500        |
    And total unbounded stake for voting 0 is 50 tokens
    And ballot for voting 0 for ExternalWorker has 50 unbounded tokens
    When 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1400         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And total unbounded stake for voting 0 is 50 tokens
    And ballot for voting 0 for ExternalWorker has 50 unbounded tokens
    When voters vote in BidEscrow formal voting with id 0
    
      | account          | REP stake | choice |
     #| ExternalWorker   | 50        | Yes    | - automatically voted by the system
      | VA1              | 500       | No     |
      | VA2              | 500       | No     |
    And 6 days passed
    And formal voting with id 0 ends in BidEscrow contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 50           | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 150          | 1000         | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | VA1              | 150          | 1000         | 0          |
      | VA2              | 150          | 1000         | 0          |
      | BidEscrow        | 0            | 0            | 0          |
    And total reputation is 3000
    And ExternalWorker is not a VA