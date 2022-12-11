Feature: Slashing voter can fully slash VA.
  VA1 crates a new KycVoter.
  VA1 participates in RepoVoter.
  VA1 creates JobOffer
  VA1 bids on another JobOffer

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | VA1              | 10000        | 1000         | 0          | true     | true  |
      | VA2              | 0            | 2000         | 0          | true     | true  |
      | VA3              | 500          | 2000         | 0          | true     | true  |
      | ExternalWorker   | 0            | 0            | 0          | true     | false |

  Scenario: VA1 gets removed from the DAO

    # VA1 crates a new voter.
    When VA1 creates random voting in KycVoter with 100 stake
    And voters vote in KycVoter informal voting with id 0
        | account | stake | vote | 
      # | VA1     | 100   | yes  | - automatically voted by the system
        | VA2     | 200   | yes  |

    # VA1 participates in another vote.
    When VA2 creates random voting in RepoVoter with 300 stake
    And voters vote in RepoVoter informal voting with id 0
        | account | stake | vote | 
        | VA1     | 200   | yes  |
      # | VA2     | 300   | yes  | - automatically voted by the system

    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 10000        | 1000         | 300        |
      | VA2              | 0            | 2000         | 500        |

    # VA1 creates a JobOffer
    When VA1 posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 100 CSPR DOS Fee
    And VA2 posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake without onboarding

    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 600          | 0            | 0          |
      | VA1              | 9900         | 1000         | 300        |
      | VA2              | 0            | 2000         | 600        |

    # VA1 participates in aother JobOffer.
    When VA3 posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 300 CSPR DOS Fee
    And VA1 posted the Bid for JobOffer 1 with proposed timeframe of 7 days and 500 CSPR price and 400 REP stake
    And VA2 posted the Bid for JobOffer 1 with proposed timeframe of 7 days and 500 CSPR price and 500 REP stake

    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 900          | 0            | 0          |
      | VA1              | 9900         | 1000         | 700        |
      | VA2              | 0            | 2000         | 1100       |
      | VA3              | 200          | 2000         | 0          |

    # TODO: VA1 is worker in active job
    # TODO: VA1 is job poster in active job
    # TODO: VA1 votes in job acceptance voting.

    # VA1 gets slashed via SlashingVoter.
    When VA2 starts voting with the following config
        | voting_contract       | stake | arg1  | arg2 |
        | SlashingVoter         | 500   | VA1   | 1    |
    And voters vote in SlashingVoter informal voting with id 0
        | account | stake | vote | 
      # | VA2     | 500   | yes  | - automatically voted by the system
        | VA3     | 500   | yes  |
    And 5 days passed
    And informal voting with id 0 ends in SlashingVoter contract
    And 2 days passed
    And voters vote in SlashingVoter formal voting with id 0
      | account | stake | vote | 
    # | VA2     | 500   | yes  | - automatically voted by the system
      | VA3     | 500   | yes  |
    And 5 days passed
    And formal voting with id 0 ends in SlashingVoter contract

    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | BidEscrow        | 300          | 0            | 0          |
      | VA1              | 10000        | 0            | 0          |
      | VA2              | 0            | 2000         | 800        |
      | VA3              | 200          | 2000         | 0          |
    And total reputation is 4000
