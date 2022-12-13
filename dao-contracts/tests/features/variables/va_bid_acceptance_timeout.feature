Feature: VA Bid
  VA can bid during external auction

  Background:
    Given accounts
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | ExternalWorker   | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                            | value   |
      | InternalAuctionTime            | 604800  |
      | ExternalAuctionTime            | 864000  |
      | VABidAcceptanceTimeout         | 172800  |
      | VotingStartAfterJobSubmission  | 0       |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee

  Scenario: Internal Worker can post a bid but cannot cancel it right away
    When InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    Then InternalWorker Bid is posted
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | InternalWorker   | 0            | 1000         | 100        |
    When InternalWorker cancels the Bid for JobPoster
    Then InternalWorker Bid is posted
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | InternalWorker   | 0            | 1000         | 100        |

  Scenario: Internal Worker can post a bid but can cancel it after 2 days
    When InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    Then InternalWorker Bid is posted
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | InternalWorker   | 0            | 1000         | 100        |
    When 2 days passed
    And InternalWorker cancels the Bid for JobPoster
    Then InternalWorker Bid isn't posted
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | InternalWorker   | 0            | 1000         | 0          |

  Scenario: External Worker can post a bid and cancel it after 2 days
    When 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 CSPR stake with onboarding
    Then ExternalWorker Bid is posted
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | ExternalWorker   | 900          | 0            | 0          |
    When 2 days passed
    And ExternalWorker cancels the Bid for JobPoster
    Then ExternalWorker Bid isn't posted
    And balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | ExternalWorker   | 1000         | 0            | 0          |