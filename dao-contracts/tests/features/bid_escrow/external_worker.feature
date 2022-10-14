Feature: External Worker submits job
  Job Poster picks a bid of an External Worker, and the External Worker accepts the job.
  The voting process is completed.

#  Background:
#    Given following starting balances
#      | account          | CSPR balance | REP balance  |
#      | Bid Escrow       | 0            | 0            |
#      | Multisig wallet  | 0            | 0            |
#      | Job Poster       | 1000         | 0            |
#      | External Worker  | 1000         | 0            |
#      | VA1              | 0            | 1000         |
#      | VA2              | 0            | 1000         |
#    And following configuration
#      | variable                   | value |
#      # how much CSPR is sent to the multisig wallet
#      | governance_payment_ratio   | 0.1   |
#      # how much REP is minted after the job is done (price * reputation_conversion_rate)
#      | reputation_conversion_rate | 0.1   |
#      # how much REP is given to voters
#      | policing_rate              | 0.3   |
#      # if the worker stake is counted as a yes vote
#      | worker_stake_as_yes_vote   | true  |
#
#  Scenario: External Worker accepts the job by staking CSPR
#    Given Job Poster picked a bid with 500 CSPR and 500 CSPR stake
#    When Internal Worker accepts the job
#    Then balances are
#      | account          | CSPR balance | REP balance  |
#      | Bid Escrow       | 500          | 50           |
#      | Multisig wallet  | 500          | 0            |
#      | Job Poster       | 500          | 0            |
#      | External Worker  | 500          | 0            |
#
#  Scenario: External Worker does the job
#    Given Job Poster picked a bid with 500 CSPR and 500 Reputation
#    And External Worker accepts the job without becoming VA
#    And External Worker does the job
#    When voting ends with votes
#    # Worker stake is NOT counted as a yes vote
#      | account          | vote | stake |
#      | VA1              | Yes  | 500   |
#      | VA2              | Yes  | 500   |
#    Then balances are
#      | account          | CSPR balance | REP balance  |
#      | Bid Escrow       | 0            | 0            |
#      | Multisig wallet  | 50           | 1            |
#      | Job Poster       | 500          | 0            |
#      | External Worker  | 315          | 85           |
#      | VA1              | 67,5         | 1007         |
#      | VA2              | 67,5         | 1007         |
#
#  Scenario: External Worker does the job and becomes VA
#    Given Job Poster picked a bid with 500 CSPR and 500 Reputation
#    And External Worker accepts the job with becoming VA
#    And External Worker does the job
#    When voting ends with votes
#    # Worker stake is NOT counted as a yes vote
#      | account          | vote | stake |
#      | VA1              | Yes  | 500   |
#      | VA2              | Yes  | 500   |
#    Then balances are
#      | account          | CSPR balance | REP balance  |
#      | Bid Escrow       | 0            | 0            |
#      | Multisig wallet  | 50           | 1            |
#      | Job Poster       | 500          | 0            |
#      | External Worker  | 18,22        | 85           |
#      | VA1              | 215,89       | 1007         |
#      | VA2              | 215,89       | 1007         |