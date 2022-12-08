Feature: VA that is subject of the slashing vote, can't participate in the voting.

  Background:
    Given following balances
      | account  | REP balance  | REP stake  | is_kyced | is_va |
      | VA1      | 1000         | 0          | true     | true  |
      | VA2      | 2000         | 0          | true     | true  |

  Scenario: VA1 can't vote
    When VA2 starts voting with the following config
        | voting_contract       | stake | arg1  | arg2 |
        | SlashingVoter         | 500   | VA1   | 0.5  |
    And voters vote in SlashingVoter's informal voting with id 0
        | account | stake | vote | 
        | VA1     | 500   | no   |
      # | VA2     | 500   | yes  | - automatically voted by the system
    Then balances are
      | account  | REP balance  | REP stake  |
      | VA1      | 1000         | 0          |
      | VA2      | 2000         | 500        |

    # And 5 days passed
    # And informal voting with id 0 ends in SlashingVoter contract
    # And 2 days passed
    # And voters vote in SlashingVoter's formal voting with id 0
    #   | account | stake | vote | 
    # # | VA2     | 500   | yes  | - automatically voted by the system
    #   | VA3     | 500   | yes  |
    #   | VA4     | 500   | no   |
    # And 5 days passed
    # And formal voting with id 0 ends in SlashingVoter contract
    # Then balances are
    #   | account          | CSPR balance | REP balance  | REP stake  |
    #   | VA1              | 0            | 500          | 0          |
    #   | VA2              | 0            | 2250         | 0          |
    #   | VA3              | 0            | 2250         | 0          |
    #   | VA4              | 0            | 1500         | 0          |
    # And total reputation is 6500
