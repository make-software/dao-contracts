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
      | key                                     | value    |
      | TimeBetweenInformalAndFormalVoting      | 86400    |
      | VotingStartAfterJobSubmission           | 0        |

    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker

    Scenario: Voting for formal voting is not possible before 24 hours after informal voting
      When InternalWorker submits the JobProof of Job 0
      And voters vote in BidEscrow informal voting with id 0
        | account          | REP stake | choice |
      # | InternalWorker   | 100       | Yes    | - automatically voted by the system
        | VA1              | 500       | Yes    |
        | VA2              | 500       | Yes    |
      And 6 days passed
      And informal voting with id 0 ends in BidEscrow contract
      Then VA1 yes vote of 500 REP fails
        | BidEscrow | 0 | formal |

    Scenario: Voting for formal voting is possible before 24 hours after informal voting
      When InternalWorker submits the JobProof of Job 0
      And voters vote in BidEscrow formal voting with id 0
        | account          | REP stake | choice |
       #| InternalWorker   | 100       | Yes  | - automatically voted by the system
        | VA1              | 500       | Yes  |
        | VA2              | 500       | Yes  |
      And 6 days passed
      And informal voting with id 0 ends in BidEscrow contract
      And 2 days passed
      Then VA1 yes vote of 500 REP succeeds
        | BidEscrow | 0 | formal |