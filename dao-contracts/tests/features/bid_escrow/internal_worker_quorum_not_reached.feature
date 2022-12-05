Feature: Internal Flow
  Job Poster picks a bid of an Internal Worker, and the Internal Worker accepts the job.
  Voting does not reach a quorum.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
      | VA3              | 0            | 1000         | 0          |
      | VA4              | 0            | 1000         | 0          |
      | VA5              | 0            | 1000         | 0          |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 100 CSPR DOS Fee
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker

  Scenario: Informal voting does not reach quorum
    When InternalWorker submits the JobProof
    And votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | Yes  | 500   |
    And Informal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And Formal voting does not start

  Scenario: Formal voting does not reach quorum
    When InternalWorker submits the JobProof
    And votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | Yes  | 500   |
      | VA2              | No   | 500   |
    And Informal voting ends
    When votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | Yes  | 500   |
    And Formal voting ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And total reputation is 6000