Feature: Slashing all of the Reputation of a VA who has not any Reputation staked

  Background:
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 2000         | 0          | true     | true  |
      | VA3              | 0            | 2000         | 0          | true     | true  |
      | VA4              | 0            | 2000         | 0          | true     | true  |

  Scenario: VA1 gets his reputation fully slashed
    When VA2 starts voting with the following config
        | voting_contract       | stake | arg1  | arg2 |
        | SlashingVoter         | 500   | VA1   | 1    |
    And voters vote in SlashingVoter's informal voting with id 0
        | account | stake | vote | 
      # | VA2     | 500   | yes  | - automatically voted by the system
        | VA3     | 500   | yes  |
        | VA4     | 500   | yes  |
    And 5 days passed
    And informal voting with id 0 ends in SlashingVoter contract
    And 2 days passed
    And voters vote in SlashingVoter's formal voting with id 0
      | account | stake | vote | 
    # | VA2     | 500   | yes  | - automatically voted by the system
      | VA3     | 500   | yes  |
      | VA4     | 500   | no   |
    And 5 days passed
    And formal voting with id 0 ends in SlashingVoter contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 0            | 0          |
      | VA2              | 0            | 2250         | 0          |
      | VA3              | 0            | 2250         | 0          |
      | VA4              | 0            | 1500         | 0          |
    And total reputation is 6000
    And VA1 is not a VA

  Scenario: Reputation slash fails
    When VA2 starts voting with the following config
        | voting_contract       | stake | arg1  | arg2 |
        | SlashingVoter         | 500   | VA1   | 1    |
    And voters vote in SlashingVoter's informal voting with id 0
        | account | stake | vote | 
      # | VA2     | 500   | yes  | - automatically voted by the system
        | VA3     | 500   | no   |
        | VA4     | 500   | no   |
    And 5 days passed
    And informal voting with id 0 ends in SlashingVoter contract
    And 2 days passed
    And voters vote in SlashingVoter's formal voting with id 0
      | account | stake | vote | 
    # | VA2     | 500   | yes  | - automatically voted by the system
      | VA3     | 500   | no   |
      | VA4     | 500   | no   |
    And 5 days passed
    And formal voting with id 0 ends in SlashingVoter contract
    Then balances are
      | account          | CSPR balance | REP balance  | REP stake  |
      | VA1              | 0            | 1000         | 0          |
      | VA2              | 0            | 1500         | 0          |
      | VA3              | 0            | 2250         | 0          |
      | VA4              | 0            | 2250         | 0          |
    And total reputation is 7000
    And VA1 is a VA
