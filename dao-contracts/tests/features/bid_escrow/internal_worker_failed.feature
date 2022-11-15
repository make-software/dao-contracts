Feature: Internal Flow
  Job Poster picks a bid of an Internal Worker, and the Internal Worker accepts the job.
  The formal voting fails.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 100 CSPR DOS Fee
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker

  Scenario: JobPoster picked the Bid of Internal Worker
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 600          | 0            | 0          |
      | JobPoster        | 400          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When InternalWorker submits the JobProof
    And votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | No   | 500   |
      | VA2              | No   | 500   |
    And Informal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 600          | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 400          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | No   | 500   |
      | VA2              | No   | 500   |
    And Formal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 900          | 0          |
      | VA1              | 0            | 1050         | 0          |
      | VA2              | 0            | 1050         | 0          |
      | BidEscrow        | 0            | 0            | 0          |
