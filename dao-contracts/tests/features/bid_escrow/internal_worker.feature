Feature: Internal Worker submits job
  Job Poster picks a bid of an Internal Worker, and the Internal Worker accepts the job.
  The voting process is completed.

  Background:
    Given following starting balances
      | account          | CSPR balance | REP balance  |
      | Bid Escrow       | 0            | 0            |
      | Multisig wallet  | 0            | 0            |
      | Job Poster       | 1000         | 0            |
      | Internal Worker  | 0            | 1000         |
      | VA1              | 0            | 1000         |
      | VA2              | 0            | 1000         |
    And following configuration
      | variable                   | value |
      # how much CSPR is sent to the multisig wallet
      | governance_payment_ratio   | 0.1   |
      # how much REP is minted after the job is done (price * reputation_conversion_rate)
      | reputation_conversion_rate | 0.1   |
      # how much REP is given to voters
      | policing_rate              | 0.3   |
      # if the worker stake is counted as a yes vote
      | worker_stake_as_yes_vote   | true  |

  Scenario: Internal Worker accepts the job by staking reputation
    Given Job Poster picked a bid with 500 CSPR and 500 Reputation
    When Internal Worker accepts the job
    Then balances are
      | account          | CSPR balance | REP balance  |
      | Bid Escrow       | 500          | 500          |
      | Job Poster       | 500          | 0            |
      | Internal Worker  | 0            | 500          |

  Scenario: Internal Worker does the job
    Given Job Poster picked a bid with 500 CSPR and 500 Reputation
    And Internal Worker accepts the job
    And Internal Worker does the job
    When voting ends with votes
    # Worker stake is NOT counted as a yes vote
      | account          | vote | stake |
      | VA1              | Yes  | 500   |
      | VA2              | Yes  | 500   |
    Then balances are:
      | account          | CSPR balance | REP balance  |
      | Bid Escrow       | 0            | 0            |
      | Multisig wallet  | 50           | 0            |
      | Job Poster       | 500          | 0            |
      | Internal Worker  | 153          | 1036         |
      | VA1              | 148,57       | 1007         |
      | VA2              | 148,57       | 1007         |

  Scenario: Internal Worker does the job #2
    Given Job Poster picked a bid with 1000 CSPR and 500 Reputation
    And Internal Worker accepts the job
    And Internal Worker does the job
    When voting ends with votes
      | account          | vote | stake |
      | VA1              | Yes  | 1000   |
      | VA2              | No   | 500   |
    Then balances are:
      | account          | CSPR balance | REP balance  |
      | Bid Escrow       | 0            | 0            |
      | Multisig wallet  | 100          | 0            |
      | Job Poster       | 500          | 0            |
      | Internal Worker  | 310,64       | 1070         |
      | VA1              | 444,19       | 1530         |
      | VA2              | 145,16       | 500          |

  Scenario: Internal Worker does the job and his stake count as a vote #3
    Given Job Poster picked a bid with 1000 CSPR and 500 Reputation
    And Internal Worker's stake counts as a vote
    And Internal Worker accepts the job
    And Internal Worker does the job
    When voting ends with votes
    # Worker stake is counted as a yes vote
      | account          | vote | stake |
      | VA1              | Yes  | 1000   |
      | VA2              | No   | 500   |
    Then balances are
      | account          | CSPR balance | REP balance  |
      | Bid Escrow       | 0            | 0            |
      | Multisig wallet | 100          | 0            |
      | Job Poster       | 500          | 0            |
      | Internal Worker | 398,84       | 1236         |
      | VA1              | 438,82       | 1363         |
      | VA2              | 161,34       | 500          |
