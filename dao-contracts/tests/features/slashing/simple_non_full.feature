Feature: Slashing a percentage of the Reputation of a VA who has not any Reputation staked

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 2000         | 0          |
      | VA3              | 0            | 2000         | 0          |
      | VA4              | 0            | 2000         | 0          |

  Scenario: VA1 gets his reputation slashed in half
    When VA2 starts slashing vote for VA1 with 500 REP stake and 50% slashing rate
    And slashing votes in informal voting 0 are
      | account          | vote | stake |
     #| VA2              | Yes  | 500   | - automatically voted by the system
      | VA3              | Yes  | 500   |
      | VA4              | Yes  | 500   |
    And slashing informal voting 0 ends
    And slashing votes in formal voting 0 are
      | account          | vote | stake |
     #| VA2              | Yes  | 500   | - automatically voted by the system
      | VA3              | Yes  | 500   |
      | VA4              | No   | 500   |
    When slashing formal voting 0 ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 500          | 0          |
      | VA2              | 0            | 2250         | 0          |
      | VA3              | 0            | 2250         | 0          |
      | VA4              | 0            | 1500         | 0          |
    And total reputation is 6500

  Scenario: Reputation slash fails
    When VA2 starts slashing vote for VA1 with 500 REP stake and 50% slashing rate
    And slashing votes in informal voting 0 are
      | account          | vote | stake |
     #| VA2              | Yes  | 500   | - automatically voted by the system
      | VA3              | No   | 500   |
      | VA4              | No   | 500   |
    And slashing informal voting 0 ends
    And slashing votes in formal voting 0 are
      | account          | vote | stake |
     #| VA2              | Yes  | 500   | - automatically voted by the system
      | VA3              | No   | 500   |
      | VA4              | No   | 500   |
    And slashing formal voting 0 ends
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1500         | 0          |
      | VA3              | 0            | 2250         | 0          |
      | VA4              | 0            | 2250         | 0          |
    And total reputation is 7000
