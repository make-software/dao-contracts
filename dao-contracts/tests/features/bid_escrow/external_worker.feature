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
    And ExternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake
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
#
#  Scenario: External Worker does the job
#    Given Job Poster picked a bid with 500 CSPR and 500 Reputation
#    And External Worker accepts the job without becoming VA
#    And External Worker does the job
#    When voting ends with votes
#    # Worker stake is NOT counted as a yes vote
#      | account          | vote | stake |
#      | VA1              | Yes  | 500   |
#      | VA2              | Yes  | 500   |
#    Then balances are
#      | account          | CSPR balance | REP balance  |
#      | Bid Escrow       | 0            | 0            |
#      | Multisig wallet  | 50           | 1            |
#      | Job Poster       | 500          | 0            |
#      | External Worker  | 315          | 85           |
#      | VA1              | 67,5         | 1007         |
#      | VA2              | 67,5         | 1007         |
#
#  Scenario: External Worker does the job and becomes VA
#    Given Job Poster picked a bid with 500 CSPR and 500 Reputation
#    And External Worker accepts the job with becoming VA
#    And External Worker does the job
#    When voting ends with votes
#    # Worker stake is NOT counted as a yes vote
#      | account          | vote | stake |
#      | VA1              | Yes  | 500   |
#      | VA2              | Yes  | 500   |
#    Then balances are
#      | account          | CSPR balance | REP balance  |
#      | Bid Escrow       | 0            | 0            |
#      | Multisig wallet  | 50           | 1            |
#      | Job Poster       | 500          | 0            |
#      | External Worker  | 18,22        | 85           |
#      | VA1              | 215,89       | 1007         |
#      | VA2              | 215,89       | 1007         |