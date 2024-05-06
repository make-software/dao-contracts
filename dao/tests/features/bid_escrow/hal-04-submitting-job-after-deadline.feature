Feature: Out of time submission
  The internal worker submits the job proof several days after the deadline.
  This is a presentation of HAL-04 issue fix.
  Background:
    Given following balances
      | account | CSPR balance | REP balance | REP stake | is_kyced | is_va |
      | BidEscrow | 1000 | 0 | 0 | false | false |
      | MultisigWallet | 0 | 0 | 0 | false | false |
      | JobPoster | 1000 | 0 | 0 | true | false |
      | InternalWorker | 0 | 1000 | 0 | true | true |
      | ExternalWorker | 500 | 0 | 0 | true | false |
      | VA1 | 0 | 1000 | 0 | true | true |
      | VA2 | 0 | 1000 | 0 | true | true |
    And following configuration
      | key | value |
      | TimeBetweenInformalAndFormalVoting | 0 |
      | VotingStartAfterJobSubmission | 0 |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 CSPR stake with onboarding
    And JobPoster picked the Bid of InternalWorker
    And 130 days passed
    Then InternalWorker fails to submit the JobProof of Job 0