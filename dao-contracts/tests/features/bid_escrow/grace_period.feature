Feature: Grace period
  Job Poster picks a bid of an Internal Worker, and the Internal Worker accepts the job.
  Job Poster does not submit a job proof in time, which starts the grace period.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | MultisigWallet   | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | ExternalWorker   | 1000         | 0            | 0          | true     | false |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                    | value |
      | TimeBetweenInformalAndFormalVoting     | 0     |
      | VotingStartAfterJobSubmission          | 0     |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee

  Scenario: Internal Worker does not submit a job proof in time and External Worker submits a job proof with onboarding
    When InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And JobPoster picked the Bid of InternalWorker
    And 8 days passed
    And ExternalWorker submits the JobProof with 1000 CSPR stake with onboarding
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 1900         | 0            | 0          |
      | MultisigWallet   | 0            | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 0            | 810          | 0          |
      | ExternalWorker   | 0            | 0            | 0          |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1000         | 0          |

  Scenario: External Worker does not submit a job proof in time, Internal Worker submits the proof and voting goes on
    When 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 CSPR stake with onboarding
    And JobPoster picked the Bid of ExternalWorker
    And 8 days passed
    And InternalWorker submits the JobProof with 100 REP stake
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | MultisigWallet   | 10           | 0            | 0          |
      | JobPoster        | 100          | 0            | 0          |
      | InternalWorker   | 30           | 1000         | 100        |
      | ExternalWorker   | 900          | 0            | 0          |
      | VA1              | 30           | 1000         | 0          |
      | VA2              | 30           | 1000         | 0          |
    When voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 500       | Yes    |
      | VA2              | 500       | Yes    |
    And 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    And voters vote in BidEscrow formal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 500       | Yes    |
      | VA2              | 500       | No     |
    And 6 days passed
    And formal voting with id 0 ends in BidEscrow contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | MultisigWallet   | 60           | 0            | 0          |
      | InternalWorker   | 195.20       | 1119.69      | 0          |
      | JobPoster        | 500          | 0            | 0          |
      | VA1              | 240.02       | 1423.48      | 0          |
      | VA2              | 104.77       | 506.82       | 0          |
      | BidEscrow        | 0            | 0            | 0          |
    And total reputation is 3050
    And ExternalWorker is not a VA
