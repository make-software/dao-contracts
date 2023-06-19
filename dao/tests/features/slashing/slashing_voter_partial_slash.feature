Feature: Slashing a percentage of the Reputation of a VA who has some Repuation staked

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 2000         | 0          | true     | true  |
      | VA3              | 0            | 2000         | 0          | true     | true  |
      | VA4              | 0            | 2000         | 0          | true     | true  |

  Scenario: VA1 gets his reputation slashed in half while he has no reputation staked
    When VA1 starts voting with the following config
        | voting_contract       | stake | arg1  | arg2 |
        | SlashingVoter         | 500   | VA4   | 0.5  |
        | SlashingVoter         | 250   | VA3   | 0.5  |
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 1000         | 750        |
    When VA2 starts voting with the following config
        | voting_contract       | stake | arg1  | arg2 |
        | SlashingVoter         | 500   | VA1   | 0.6  |
    And voters vote in SlashingVoter informal voting with id 2
        | account | stake | vote | 
      # | VA2     | 500   | yes  | - automatically voted by the system
        | VA3     | 500   | yes  |
        | VA4     | 500   | yes  |
    And 5 days passed
    And informal voting with id 2 ends in SlashingVoter contract
    And 2 days passed
    And voters vote in SlashingVoter formal voting with id 2
      | account | stake | vote | 
    # | VA2     | 500   | yes  | - automatically voted by the system
      | VA3     | 500   | yes  |
      | VA4     | 500   | no   |
    And 5 days passed
    And formal voting with id 2 ends in SlashingVoter contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 400          | 750        |
      | VA2              | 0            | 2250         | 0          |
      | VA3              | 0            | 2250         | 0          |
      | VA4              | 0            | 1500         | 0          |
    And total reputation is 6400
    When informal voting with id 0 ends in SlashingVoter contract
    And informal voting with id 1 ends in SlashingVoter contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 400          | 0          |
      | VA2              | 0            | 2250         | 0          |
      | VA3              | 0            | 2250         | 0          |
      | VA4              | 0            | 1500         | 0          |
