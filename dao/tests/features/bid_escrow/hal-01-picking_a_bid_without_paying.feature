Feature: Picking a bid without paying
  JobPoster posts a job, internal worker is bidding.
  Job Poster picks a bid of an Internal Worker, without sending exact amount of CSPR.
  Picking a bid is rejected.
  This is a presentation of HAL-01 issue fix.
  Background:
    Given following balances
      | account | CSPR balance | REP balance | REP stake | is_kyced | is_va |
      | BidEscrow | 0 | 0 | 0 | false | false |
      | MultisigWallet | 0 | 0 | 0 | false | false |
      | JobPoster | 1000 | 0 | 0 | true | false |
      | InternalWorker | 0 | 1000 | 0 | true | true |
      | VA1 | 0 | 1000 | 0 | true | true |
      | VA2 | 0 | 1000 | 0 | true | true |
    And following configuration
      | key | value |
      | TimeBetweenInformalAndFormalVoting | 0 |
      | VotingStartAfterJobSubmission | 0 |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    Then balances are
      | account | CSPR balance | REP balance | REP stake |
      # Initial 1000 + 400 dos fee. Notice lack of 500 CSPR payment from the bid.
      | BidEscrow | 400 | 0 | 0 |
      | JobPoster | 600 | 0 | 0 |
      | InternalWorker | 0 | 1000 | 100 |
      | VA1 | 0 | 1000 | 0 |
      | VA2 | 0 | 1000 | 0 |
  Scenario: JobPoster picked the Bid of InternalWorker without sending exact amount of CSPR
    # Following step will fail, before fix it would pass
    When JobPoster picked the Bid without paying for InternalWorker
    Then balances are
      | account | CSPR balance | REP balance | REP stake |
      # Initial 1000 + 400 dos fee. Notice lack of 500 CSPR payment from the bid.
      | BidEscrow | 400 | 0 | 0 |
      | JobPoster | 600 | 0 | 0 |
      | InternalWorker | 0 | 1000 | 100 |
      | VA1 | 0 | 1000 | 0 |
      | VA2 | 0 | 1000 | 0 |
    And the Bid of InternalWorker is in state Created