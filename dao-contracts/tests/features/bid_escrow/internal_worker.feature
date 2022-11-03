Feature: Internal Flow
  Job Poster picks a bid of an Internal Worker, and the Internal Worker accepts the job.
  The voting process is completed.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 1000         | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |

  Scenario: Internal Worker accepts the job by staking reputation
    Given JobPoster picked a bid with 500 CSPR and 500 Reputation for InternalWorker
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 500          | 0            | 500        |
      | JobPoster        | 500          | 0            | 0          |
      | InternalWorker   | 0            | 1000         | 500        |

  Scenario: Internal Worker does the job
    Given JobPoster picked a bid with 500 CSPR and 500 Reputation for InternalWorker
    And Internal Worker does the job
    When voting ends with votes
    # Worker stake is NOT counted as a yes vote
      | account          | vote | stake |
      | VA1              | Yes  | 500   |
      | VA2              | Yes  | 500   |
    Then balances are:
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 0            | 0            | 0          |
      | MultisigWallet   | 50           | 0            | 0          |
      | JobPoster        | 500          | 0            | 0          |
      | InternalWorker   | 153          | 1036         | 0          |
      | VA1              | 148,57       | 1007         | 0          |
      | VA2              | 148,57       | 1007         | 0          |

  Scenario: Internal Worker does the job #2
    Given JobPoster picked a bid with 1000 CSPR and 500 Reputation for InternalWorker
    And Internal Worker does the job
    When voting ends with votes
      | account          | vote | stake |
      | VA1              | Yes  | 1000  |
      | VA2              | No   | 500   |
    Then balances are:
      | account         | CSPR balance | REP balance  | REP stake  |
      | BidEscrow       | 0            | 0            | 0          |
      | MultisigWallet  | 100          | 0            | 0          |
      | JobPoster       | 500          | 0            | 0          |
      | InternalWorker  | 310,64       | 1070         | 0          |
      | VA1             | 444,19       | 1530         | 0          |
      | VA2             | 145,16       | 500          | 0          |