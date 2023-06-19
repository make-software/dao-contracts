Feature: VA that is subject of the slashing vote, can't participate in the voting.

  Scenario: VA1 can't vote
    Given following balances
      | account  | REP balance  | REP stake  | is_kyced | is_va |
      | VA1      | 1000         | 0          | true     | true  |
      | VA2      | 2000         | 0          | true     | true  |
    When VA2 starts voting with the following config
        | voting_contract       | stake | arg1  | arg2 |
        | SlashingVoter         | 500   | VA1   | 0.5  |
    And voters vote in SlashingVoter informal voting with id 0
        | account | stake | vote | 
        | VA1     | 500   | no   |
      # | VA2     | 500   | yes  | - automatically voted by the system
    Then balances are
      | account  | REP balance  | REP stake  |
      | VA1      | 1000         | 0          |
      | VA2      | 2000         | 500        |

