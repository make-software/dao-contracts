Feature: TimeBetweenInformalAndFormalVoting Variable
  TimeBetweenInformalAndFormalVoting tests

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    And following configuration
      | key                                     | value        |
      | TimeBetweenInformalAndFormalVoting      | 86400        |
    And JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 100 CSPR DOS Fee
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker

    Scenario: Voting for formal voting is not possible before 24 hours after informal voting
      When InternalWorker submits the JobProof
      And votes are
        | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
        | VA1              | Yes  | 500   |
        | VA2              | Yes  | 500   |
      And Informal voting ends
      And VA1 yes vote of 500 REP fails

  Scenario: Voting for formal voting is possible before 24 hours after informal voting
    When InternalWorker submits the JobProof
    And votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | Yes  | 500   |
      | VA2              | Yes  | 500   |
    And Informal voting ends
    And 2 days passed
    And VA1 yes vote of 500 REP succeeds