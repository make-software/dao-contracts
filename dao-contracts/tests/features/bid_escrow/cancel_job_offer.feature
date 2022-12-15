Feature: Cancelling the Job Offer
  Job Poster can cancel the Job after the auction phase.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee

  Scenario: Nobody submits a bid, JobPoster cancels the Job during auction phase
    When JobPoster cancels the JobOffer with id 0
    Then JobOffer with id 0 isn't cancelled
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | JobPoster        | 600          | 0            | 0          |

  Scenario: JobPoster cancels the Job after the auction phase
    When 18 days passed
    And JobPoster cancels the JobOffer with id 0
    Then JobOffer with id 0 is cancelled
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | JobPoster        | 1000         | 0            | 0          |

  Scenario: Somebody tries to cancel the Job after the auction phase
    When 18 days passed
    And VA1 cancels the JobOffer with id 0
    Then JobOffer with id 0 isn't cancelled
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | JobPoster        | 600          | 0            | 0          |