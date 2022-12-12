Feature: Full bid slash on BidEscrow
    VA can have his bids and created jobs offers removed.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | true  |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | ExternalWorker   | 500          | 0            | 0          | true     | false |
      | VA1              | 0            | 300          | 0          | true     | true  |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
      | VotingStartAfterJobSubmission          | 0             |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And VA1 posted the Bid for JobOffer 0 with proposed timeframe of 2 days and 100 CSPR price and 200 REP stake
    And 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding

  Scenario: JobPoster gets slashed
    When JobPoster got his active job offers slashed
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | ExternalWorker   | 500          | 0            | 0          |
      | VA1              | 0            | 300          | 0          |

  Scenario: InternalWorker gets slashed
    When bid with id 0 is slashed
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | JobPoster        | 600          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | VA1              | 0            | 300          | 200        |
