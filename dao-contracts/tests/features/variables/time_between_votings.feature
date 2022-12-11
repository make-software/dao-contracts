Feature: TimeBetweenInformalAndFormalVoting Variable
  TimeBetweenInformalAndFormalVoting tests

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                     | value        |
      | TimeBetweenInformalAndFormalVoting      | 86400        |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 100 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker

    Scenario: Voting for formal voting is not possible before 24 hours after informal voting
      When InternalWorker submits the JobProof
      And votes are
        | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
        | VA1              | Yes  | 500   |
        | VA2              | Yes  | 500   |
      And Informal voting ends
      Then VA1 yes vote of 500 REP fails

    Scenario: Voting for formal voting is possible before 24 hours after informal voting
      When InternalWorker submits the JobProof
      And votes are
        | account          | vote | stake |
       #| InternalWorker   | Yes  | 100   | - automatically voted by the system
        | VA1              | Yes  | 500   |
        | VA2              | Yes  | 500   |
      And Informal voting ends
      And 2 days passed
      Then VA1 yes vote of 500 REP succeeds