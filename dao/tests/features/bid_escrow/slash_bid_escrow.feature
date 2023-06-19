Feature: Slash on BidEscrow
    VA can have his bids and created jobs offers removed.

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | true  |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | ExternalWorker   | 500          | 0            | 0          | true     | false |
      | VA1              | 0            | 300          | 0          | true     | true  |
      | VA2              | 0            | 100          | 0          | true     | true  |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
      | VotingStartAfterJobSubmission          | 0             |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And VA1 posted the Bid for JobOffer 0 with proposed timeframe of 2 days and 100 CSPR price and 200 REP stake
    And 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding
    And Owner adds Alice to whitelist in BidEscrow contract
 
  # Scenario: JobPoster gets slashed during job bidding
  #   When Alice calls BidEscrow to slash JobPoster
  #   Then balances are
  #     | account          | CSPR balance | REP balance  | REP stake  |
  #     | BidEscrow        | 0            | 0            | 0          |
  #     | JobPoster        | 1000         | 0            | 0          |
  #     | InternalWorker   | 0            | 1000         | 0          |
  #     | ExternalWorker   | 500          | 0            | 0          |
  #     | VA1              | 0            | 300          | 0          |

  # Scenario: InternalWorker gets slashed during job bidding
  #   When Alice calls BidEscrow to slash InternalWorker
  #   Then balances are
  #     | account          | CSPR balance | REP balance  | REP stake  |
  #     | BidEscrow        | 900          | 0            | 0          |
  #     | JobPoster        | 600          | 0            | 0          |
  #     | InternalWorker   | 0            | 1000         | 0          |
  #     | ExternalWorker   | 0            | 0            | 0          |
  #     | VA1              | 0            | 300          | 200        |

  # Scenario: InternalWorker gets slashed while working
  #   When JobPoster picked the Bid of InternalWorker
  #   When Alice calls BidEscrow to slash InternalWorker
  #   Then balances are
  #     | account          | CSPR balance | REP balance  | REP stake  |
  #     | BidEscrow        | 0            | 0            | 0          |
  #     | JobPoster        | 1000         | 0            | 0          |
  #     | InternalWorker   | 0            | 900          | 0          |
  #     | ExternalWorker   | 500          | 0            | 0          |
  #     | VA1              | 0            | 300          | 0          |

  Scenario: InternalWorker gets slashed during voting
    When JobPoster picked the Bid of InternalWorker
    And InternalWorker submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| InternalWorker   | 100       | Yes    | - automatically voted by the system
      | VA1              | 100       | Yes    |
    And 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    And voters vote in BidEscrow formal voting with id 0
      | account          | REP stake | choice |
      | VA1              | 100       | Yes    |
      | VA2              | 100       | No     |
    And Alice calls BidEscrow to slash InternalWorker
    # Then balances are
    #   | account          | CSPR balance | REP balance  | REP stake  |
    #   | BidEscrow        | 0            | 0            | 0          |
    #   | JobPoster        | 1000         | 0            | 0          |
    #   | InternalWorker   | 0            | 900          | 0          |
    #   | ExternalWorker   | 500          | 0            | 0          |
    #   | VA1              | 0            | 500          | 0          |
    #   | VA2              | 0            | 100          | 0          |
