Feature: Internal Flow
  Job Poster picks a bid of an Internal Worker, and the Internal Worker accepts the job.
  The voting process is completed.

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
    And JobPoster picks the Bid of InternalWorker


  Scenario: JobPoster picked the Bid of Internal Worker
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 500          | 0            | 0          |
      | JobPoster        | 500          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
#    When InternalWorker submits the JobProof
#    And Informal voting ends with votes
#      | account          | vote | stake |
#      | InternalWorker   | Yes  | 100   |
#      | VA1              | Yes  | 500   |
#      | VA2              | Yes  | 500   |
#    Then balances are:
#      | account          | CSPR balance | REP balance  | REP stake  |
#      | BidEscrow        | 500          | 0            | 0          |
#      | MultisigWallet   | 0            | 0            | 0          |
#      | JobPoster        | 500          | 0            | 0          |
#      | InternalWorker   | 0            | 1000         | 100        |
#      | VA1              | 0            | 1000         | 0          |
#      | VA2              | 0            | 1000         | 0          |
#    When FormalVoting ends with votes
#      | account          | vote | stake |
#      | InternalWorker   | Yes  | 100   |
#      | VA1              | Yes  | 500   |
#      | VA2              | No   | 500   |
#    Then balances are:
#      | account          | CSPR balance | REP balance  | REP stake  |
#      | BidEscrow        | 0            | 0            | 0          |
#      | MultisigWallet   | 50           | 0            | 0          |
#      | JobPoster        | 500          | 0            | 0          |
#      | InternalWorker   | 183.83       | 1121         | 0          |
#      | VA1              | 233.19       | 1422         | 0          |
#      | VA2              | 82.97        | 506          | 0          |