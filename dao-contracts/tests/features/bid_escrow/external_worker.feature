Feature: External Worker who wants to become a VA submits job
  Job Poster picks a bid of an External Worker, and the External Worker does the job.
  The External Worker wants to become a VA.
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
    And ExternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of ExternalWorker

  Scenario: JobPoster picked the Bid of Internal Worker
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1100         | 0            | 0          |
      | JobPoster        | 400          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When ExternalWorker submits the JobProof
    And Informal voting ends with votes
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
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When Formal voting ends with votes
      | account          | vote | stake |
     #| ExternalWorker   | Yes  | 50    | - automatically voted by the system
      | VA1              | Yes  | 500   |
      | VA2              | No   | 500   |
    Then total_unbounded_stake is 0
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 100          | 0            | 0          |
      | JobPoster        | 500          | 0            | 0          |
      | InternalWorker   | 290.32       | 1000         | 0          |
      | ExternalWorker   | 38.07        | 131.16       | 0          |
      | VA1              | 424.35       | 1461.68      | 0          |
      | VA2              | 147.23       | 507.14       | 0          |
      | BidEscrow        | 0            | 0            | 0          |
    And ExternalWorker is a VA