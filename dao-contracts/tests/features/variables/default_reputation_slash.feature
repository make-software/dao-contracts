Feature: Default reputation slash

  If an Internal Worker does not complete the job, the DefaultReputationSlash
  indicates how much of his reputation is getting slashed.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                    | value |
      | TimeBetweenInformalAndFormalVoting     | 0     |
      | DefaultReputationSlash                 | 250   |
      | VotingStartAfterJobSubmission          | 0     |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker
    And InternalWorker submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 500       | No     |
      | VA2              | 500       | No     |
    And 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    And voters vote in BidEscrow formal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 500       | No     |
      | VA2              | 500       | No     |
    And 6 days passed
    And formal voting with id 0 ends in BidEscrow contract

  Scenario: Internal Worker losses 25% of his remaining reputation, if the job is undone
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | InternalWorker   | 0            | 675          | 0          |
    And total reputation is 2775
