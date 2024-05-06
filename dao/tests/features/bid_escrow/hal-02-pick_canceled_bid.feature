Feature: Pick a canceled bid
  JobPoster cannot pick a bid that has already been cancelled.
This is a presentation of HAL-02 issue fix.
  Background:
    Given accounts
      | account | CSPR balance | REP balance | REP stake | is_kyced | is_va |
      | JobPoster | 1000 | 0 | 0 | true | false |
      | Alice | 1000 | 0 | 0 | true | false |
      | ExternalWorker | 1000 | 0 | 0 | true | false |
      | InternalWorker | 0 | 1000 | 0 | true | true |
      | VA1 | 0 | 1000 | 0 | true | true |
      | VA2 | 0 | 1000 | 0 | true | true |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And 2 days passed
    And Alice posted a JobOffer with expected timeframe of 20 days, maximum budget of 2000 CSPR and 400 CSPR DOS Fee
  Scenario: JobPoster try to pick the cancelled bid
    When InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    Then InternalWorker Bid is posted
    And balances are
      | account | CSPR balance | REP balance | REP stake |
      | InternalWorker | 0 | 1000 | 100 |
    When InternalWorker posted the Bid for JobOffer 1 with proposed timeframe of 7 days and 500 CSPR price and 200 REP stake
    Then InternalWorker Bid is posted
    And balances are
      | account | CSPR balance | REP balance | REP stake |
      | InternalWorker | 0 | 1000 | 300 |
    When 2 days passed
    And InternalWorker cancels the Bid for JobPoster
    Then Bid 0 is canceled
    And balances are
      | account | CSPR balance | REP balance | REP stake |
      | InternalWorker | 0 | 1000 | 200 |
    When 1 days passed
    Then JobPoster fails to pick the Bid of InternalWorker
    