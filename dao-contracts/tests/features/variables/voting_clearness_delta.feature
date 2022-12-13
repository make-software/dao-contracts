Feature: Voting clearness delta
  If VotingClearnessDelta is set to 8 and the result of the Informal Voting is 42 percent “for” and 58 “against”
  then the time between votings should be doubled.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                    | value         |
      | VotingClearnessDelta                   | 8             |
      | TimeBetweenInformalAndFormalVoting     | 86400         |
      | VotingStartAfterJobSubmission          | 0             |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker

  Scenario: Results are far away - time between votings stays the same
    When InternalWorker submits the JobProof
    And votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | Yes  | 250   |
      | VA2              | No   | 750   |
    And Informal voting ends
    Then VA1 yes vote of 500 REP fails
    When 1 day passed
    Then VA1 yes vote of 500 REP succeeds
  Scenario: Results are close - time between votings is doubled
    When InternalWorker submits the JobProof
    And votes are
      | account          | vote | stake |
     #| InternalWorker   | Yes  | 100   | - automatically voted by the system
      | VA1              | Yes  | 450   |
      | VA2              | No   | 550   |
    And Informal voting ends
    And 1 day passed
    Then VA1 yes vote of 500 REP fails
    When 1 day passed
    Then VA1 yes vote of 500 REP succeeds