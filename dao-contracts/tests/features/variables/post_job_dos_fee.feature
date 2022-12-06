Feature: PostJobDosFee Variable
  To post Job, Job Poster needs to send a dos fee not less than $10

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
    And following configuration
      | key              | value        |
      | PostJobDOSFee    | 10           |
    And the price of USDT is 21 CSPR

  Scenario: Post a job with insufficient dos fee
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 200 CSPR DOS Fee
    Then the JobOffer isn't posted

  Scenario: Post a job with sufficient dos fee
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 220 CSPR DOS Fee
    Then the JobOffer is posted