Feature: External Worker who doesn't want to become a VA submits job
  Job Poster picks a bid of an External Worker, and the External Worker does the job.
  The External Worker doesn't want to become a VA.
  The voting process is completed.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | ExternalWorker   | 500          | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 100 CSPR DOS Fee
    And ExternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake without onboarding
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of ExternalWorker

  Scenario: JobPoster picked the Bid of External Worker
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1100         | 0            | 0          |
      | JobPoster        | 400          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When ExternalWorker submits the JobProof
    And votes are
      | account          | vote | stake |
     #| ExternalWorker   | Yes  | 50   | - automatically voted by the system - 500CSPR converted to 50 Reputation
      | VA1              | Yes  | 500   |
      | VA2              | No   | 500   |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1100         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 400          | 0            | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 500        |
      | VA2              | 0            | 1000         | 500        |
    And total unbounded stake for informal voting 0 is 50 tokens
    And ballot for informal voting 0 for ExternalWorker has 50 unbounded tokens
    When Informal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1100         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 400          | 0            | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And total unbounded stake for formal voting 0 is 50 tokens
    And ballot for formal voting 0 for ExternalWorker has 50 unbounded tokens
    When votes are
      | account          | vote | stake |
     #| ExternalWorker   | Yes  | 50    | - automatically voted by the system
      | VA1              | Yes  | 500   |
      | VA2              | No   | 500   |
    And Formal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 50           | 0            | 0          |
      | JobPoster        | 500          | 0            | 0          |
      | InternalWorker   | 44.77        | 1000         | 0          |
      | ExternalWorker   | 815          | 0            | 0          |
      | VA1              | 67.5         | 1507.5       | 0          |
      | VA2              | 22.72        | 507.5        | 0          |
      | BidEscrow        | 0            | 0            | 0          |
    And total reputation is 3015
    And ExternalWorker is not a VA