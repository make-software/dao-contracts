Feature: External Worker who doesn't want to become a VA submits job
  Job Poster picks a bid of an External Worker, and the External Worker does the job.
  The External Worker doesn't want to become a VA.
  The formal voting fails.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | MultisigWallet   | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 1000         | 0          | true     | true  |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | ExternalWorker   | 500          | 0            | 0          | true     | false |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And ExternalWorker posted the Bid with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake without onboarding
    # And JobPoster picked the Bid of ExternalWorker

  Scenario: JobPoster is fully slashed
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  | 
      | BidEscrow        | 600          | 0            | 0          |
      | JobPoster        | 900          | 1000         | 0          |
      | InternalWorker   | 0            | 1000         | 100        |
      | ExternalWorker   | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
    When VA1 starts voting with the following config
      | voting_contract       | stake | arg1       | arg2 |
      | SlashingVoter         | 1000  | JobPoster  | 1    |
    And voters vote in SlashingVoter's informal voting with id 0
      | account | stake | vote |
    # | VA1     | 1000  | yes  | - automatically voted by the system
      | VA2     | 1000  | yes  |
    When 5 days passed
    And informal voting with id 0 ends in SlashingVoter contract
    And 2 days passed
    And voters vote in SlashingVoter's formal voting with id 0
      | account | stake | vote |
    # | VA1     | 1000  | yes  | - automatically voted by the system
      | VA2     | 1000  | yes  |
    And 5 days passed
    And formal voting with id 0 ends in SlashingVoter contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  | 
      | BidEscrow        | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | ExternalWorker   | 500          | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |
