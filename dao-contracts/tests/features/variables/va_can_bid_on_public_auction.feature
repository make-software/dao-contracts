Feature: VA Bid
  VA can bid during external auction

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | ExternalWorker   | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                     | value        |
      | InternalAuctionTime     | 604800       |
      | ExternalAuctionTime     | 864000       |
      | VACanBidOnPublicAuction | true         |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee

  Scenario: Internal Worker can post a bid within InternalAuctionTime but External Worker cannot
    When ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    Then InternalWorker Bid is posted
    And ExternalWorker Bid isn't posted

  Scenario: External Worker and Internal Worker can post bid on External Auction when variable is set
    When 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    Then InternalWorker Bid is posted
    And ExternalWorker Bid is posted
