Feature: VA Bid
  VA can bid during external auction

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | ExternalWorker   | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
    And following configuration
      | key                     | value        |
      | InternalAuctionTime     | 604800       |
      | ExternalAuctionTime     | 864000       |
      | VACanBidOnPublicAuction | true         |
    And JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 100 CSPR DOS Fee

  Scenario: Internal Worker can post a bid within InternalAuctionTime but External Worker cannot
    Given ExternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    Then InternalWorker Bid is posted
    And ExternalWorker Bid isn't posted

  Scenario: External Worker can post bid on External Auction, but Internal cannot
    When 8 days passed
    Given ExternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    Then ExternalWorker Bid is posted
    And InternalWorker Bid is posted
