Feature: AuctionTime Variables
  InternalAuctionTime and ExternalAuctionTime variables tests

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | ExternalWorker   | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
    And following configuration
      | key                     | value        |
      # TODO: change to "7 days"
      | InternalAuctionTime     | 604800       |
      # TODO: change to "10 days"
      | ExternalAuctionTime     | 864000       |
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
    Then InternalWorker Bid isn't posted
    And ExternalWorker Bid is posted