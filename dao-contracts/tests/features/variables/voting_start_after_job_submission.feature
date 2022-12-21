Feature: Voting start after job submission
  VotingStartAfterJobSubmission variable tests

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | VA1              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                           | value        |
    # | VotingStartAfterJobSubmission | 2 days       |
      | VotingStartAfterJobSubmission | 172800       |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker
    And InternalWorker submits the JobProof of Job 0

  Scenario: Cannot vote before the voting is started
    When 1 day passed
    Then VA1 yes vote of 500 REP fails
      | BidEscrow | 0 | informal |

  Scenario: Voting is allowed once voting is started
    When 2 days passed
    Then VA1 yes vote of 500 REP succeeds
      | BidEscrow | 0 | informal |
