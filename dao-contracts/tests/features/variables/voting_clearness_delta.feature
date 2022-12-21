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
    When InternalWorker submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 250       | Yes    |
      | VA2              | 750       | No     |
    And 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    Then VA1 yes vote of 500 REP fails
      | BidEscrow | 0 | formal |
    When 1 day passed
    Then VA1 yes vote of 500 REP succeeds
      | BidEscrow | 0 | formal |
  Scenario: Results are close - time between votings is doubled
    When InternalWorker submits the JobProof of Job 0
    And voters vote in BidEscrow formal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 450       | Yes    |
      | VA2              | 550       | No     |
    And 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    And 1 day passed
    Then VA1 yes vote of 500 REP fails
      | BidEscrow | 0 | formal |
    When 1 day passed
    Then VA1 yes vote of 500 REP succeeds
      | BidEscrow | 0 | formal |