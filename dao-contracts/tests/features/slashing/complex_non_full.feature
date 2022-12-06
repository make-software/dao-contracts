Feature: Slashing a percentage of the Reputation of a VA who has some Repuation staked

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 2000         | 0          | true     | true  |
      | VA3              | 0            | 2000         | 0          | true     | true  |
      | VA4              | 0            | 2000         | 0          | true     | true  |

  Scenario: VA1 gets his reputation slashed in half while he has no reputation staked
    # When VA1 starts slashing vote for VA4 with 500 REP stake and 50% slashing rate
    When VA1 starts voting with the following config
        | voting_contract       | stake | arg1  | arg2 |
        | SlashingVoter         | 500   | VA4   | 0.5  |
    And VA1 starts slashing vote for VA3 with 250 REP stake and 50% slashing rate
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 1000         | 750        |
    When VA2 starts slashing vote for VA1 with 500 REP stake and 50% slashing rate
    And slashing votes in informal voting 2 are
      | account          | vote | stake |
     #| VA2              | Yes  | 500   | - automatically voted by the system
      | VA3              | Yes  | 500   |
      | VA4              | Yes  | 500   |
    And slashing informal voting 2 ends
    And slashing votes in formal voting 2 are
      | account          | vote | stake |
     #| VA2              | Yes  | 500   | - automatically voted by the system
      | VA3              | Yes  | 500   |
      | VA4              | No   | 500   |
    When slashing formal voting 2 ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 500          | 750        |
      | VA2              | 0            | 2250         | 0          |
      | VA3              | 0            | 2250         | 0          |
      | VA4              | 0            | 1500         | 0          |
    And total reputation is 6500
    When slashing informal voting 0 ends
    And slashing informal voting 1 ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 500          | 0          |
      | VA2              | 0            | 2250         | 0          |
      | VA3              | 0            | 2250         | 0          |
      | VA4              | 0            | 1500         | 0          |