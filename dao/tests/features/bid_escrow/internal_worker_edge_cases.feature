Feature: Internal Worker edge cases
  External and Internal Workers are submitting a bid.
  Job Poster picks a bid of an Internal Worker, and the Internal Worker accepts the job.
  External Worker's stake is returned.
  The voting process is completed.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | MultisigWallet   | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 1000         | 1000         | 0          | true     | true  |
      | ExternalWorker   | 500          | 0            | 0          | true     | false |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
      | VotingStartAfterJobSubmission          | 0             |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee

  Scenario: Internal Worker sends both CSPR and REP as a stake for submitting the bid
    When InternalWorker posted the Bid for JobOffer 0 with 100 CSPR and 100 REP staked
    Then balances are
        | account          | CSPR balance | REP balance  | REP stake  |
        | BidEscrow        | 400          | 0            | 0          |
        # Because it shouldn't work and thus change anything
        | InternalWorker   | 1000         | 1000         | 0          |