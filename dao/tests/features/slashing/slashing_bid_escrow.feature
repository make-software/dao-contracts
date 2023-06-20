Feature: Slashing in Bid Escrow contract

  Background:
    Given following balances
      | account           | REP balance | CSPR balance  | is_kyced  | is_va |
      | Alice             | 0           | 0             | false     | false |
      | BidEscrow         | 0           | 0             | false     | false |
      | JobPoster         | 0           | 1000          | true      | false |
      | VA1               | 1000        | 1000          | true      | true  |
      | VA2               | 1000        | 0             | true      | true  |
      | VA3               | 1000        | 0             | true      | true  |
      | VA4               | 1000        | 0             | true      | true  |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
      | VotingStartAfterJobSubmission          | 0             |

  Scenario: VA1 gets slashed while being a worker in active job
    When Owner adds Alice to whitelist in BidEscrow contract
    And JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And VA1 posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And JobPoster picked the Bid of VA1
    And VA1 submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| VA1              | 100       | Yes    | - automatically voted by the system
      | VA2              | 500       | Yes    |
      | VA3              | 500       | Yes    |
    And Alice calls BidEscrow to slash VA1
    Then balances are
      | account           | REP balance | CSPR balance  |
      | Alice             | 0           | 0             |
      # JobPoster cspr is returned
      | JobPoster         | 0           | 1000          |
      # We did not perform voting, so the reputation is not burned
      # All reps is returned
      | VA1               | 1000        | 1000          |
      | VA2               | 1000        | 0             |
      | VA3               | 1000        | 0             |
    And total reputation is 4000

  Scenario: VA1 gets slashed while being a JobPoster in active job
    When Owner adds Alice to whitelist in BidEscrow contract
    And VA1 posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And VA2 posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And VA1 picked the Bid of VA2
    And VA2 submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| VA2              | 100       | Yes    | - automatically voted by the system
      | VA3              | 500       | Yes    |
    And Alice calls BidEscrow to slash VA1
    # Despite slashing, the process should continue as normal
    And 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    When voters vote in BidEscrow formal voting with id 0
      | account          | REP stake | choice |
     #| VA2              | 100       | Yes    | - automatically voted by the system
      | VA3              | 500       | Yes    |
    And 6 days passed
    And formal voting with id 0 ends in BidEscrow contract
    Then balances are
      | account           | REP balance | CSPR balance  |
      | Alice             | 0           | 0             |
      | JobPoster         | 0           | 1000          |
      | VA2               | 1037.50     | 115.27        |
      | VA3               | 1012.5      | 112.5         |
      | VA1               | 1000       | 611.11          |
    And total reputation is 4050

  Scenario: VA1 votes in formal job acceptance voting
    When Owner adds Alice to whitelist in BidEscrow contract
    And Owner adds Alice to whitelist in ReputationToken contract
    And JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And VA2 posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And JobPoster picked the Bid of VA2
    And VA2 submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
      | VA1              | 500       | Yes    |
     #| VA2              | 100       | Yes    | - automatically voted by the system
      | VA3              | 500       | Yes    |
      | VA4              | 500       | Yes    |
    And 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    When voters vote in BidEscrow formal voting with id 0
      | account          | REP stake | choice |
     #| VA2              | 100       | Yes    | - automatically voted by the system
      | VA1              | 500       | Yes    |
      | VA3              | 500       | Yes    |
      | VA4              | 500       | No     |
    And Alice calls BidEscrow to slash VA1
    # We burn all reputation, this mimicks the whole slashing voting process
    And Alice burns all reputation of VA1
    And 6 days passed
    And formal voting with id 0 ends in BidEscrow contract
    Then balances are
      | account           | REP balance | CSPR balance  |
      | Alice             | 0           | 0             |
      | JobPoster         | 0           | 500           |
      | VA2               | 1119.69     | 165.20        |
      | VA3               | 1423.48     | 210.02        |
      | VA4               | 506.82      | 74.77         |
      | VA1               | 0           | 1000          |
    And total reputation is 3050
