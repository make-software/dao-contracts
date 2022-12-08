Feature: Slashing all of the Reputation of a VA who has pariticipated in a voting.

  Background:
    Given following balances
      | account | REP balance  | REP stake  | is_kyced | is_va |
      | VA1     | 1000         | 0          | true     | true  |
      | VA2     | 2000         | 0          | true     | true  |
      | VA3     | 2000         | 0          | true     | true  |
      | VA4     | 2000         | 0          | true     | true  |
    When VA2 starts voting with the following config
      | voting_contract  | stake | arg1    |
      | KycVoter         | 500   | Alice   |
    And voters vote in KycVoter's informal voting with id 0
      | account | stake | vote | 
      | VA1     | 500   | yes  |
    # | VA2     | 500   | yes  | - automatically voted by the system
      | VA3     | 500   | yes  |
      | VA4     | 500   | yes  |
    Then balances are
      | account | REP balance  | REP stake  |
      | VA1     | 1000         | 500        |
      | VA2     | 2000         | 500        |
      | VA3     | 2000         | 500        |
      | VA4     | 2000         | 500        |
    When VA2 starts voting with the following config
      | voting_contract       | stake | arg1  | arg2 |
      | SlashingVoter         | 1500  | VA1   | 1    |
    And voters vote in SlashingVoter's informal voting with id 0
      | account | stake | vote |
    # | VA2     | 1500  | yes  | - automatically voted by the system
      | VA3     | 500   | yes  |
      | VA4     | 500   | yes  |
    Then balances are
      | account | REP balance  | REP stake  |
      | VA1     | 1000         | 500        |
      | VA2     | 2000         | 2000       |
      | VA3     | 2000         | 1000       |
      | VA4     | 2000         | 1000       |
    When 5 days passed
    And informal voting with id 0 ends in SlashingVoter contract
    And 2 days passed
    And voters vote in SlashingVoter's formal voting with id 0
      | account | stake | vote | 
    # | VA2     | 1500  | yes  | - automatically voted by the system
      | VA3     | 500   | yes  |
      | VA4     | 500   | no   |

  Scenario: VA1 gets his reputation fully slashed and his votes is undone in informal voting.
    When 5 days passed
    And formal voting with id 0 ends in SlashingVoter contract
    Then balances are
      | account | REP balance  | REP stake  |
      | VA1     | 0            | 0          |
      | VA2     | 2375         | 500        |
      | VA3     | 2125         | 500        |
      | VA4     | 1500         | 500        |
    And total reputation is 6000
    And voting with id 0 in KycVoter is canceled.

  Scenario: VA1 gets his reputation fully slashed and his votes is undone in formal voting.
    When informal voting with id 0 ends in KycVoter contract
    And voters vote in KycVoter's formal voting with id 0
      | account | stake | vote | 
      | VA1     | 500   | yes  |
    # | VA2     | 500   | yes  | - automatically voted by the system
      | VA3     | 500   | yes  |
      | VA4     | 500   | yes  |
    And 5 days passed
    And formal voting with id 0 ends in SlashingVoter contract
    Then balances are
      | account | REP balance  | REP stake  |
      | VA1     | 0            | 0          |
      | VA2     | 2375         | 500        |
      | VA3     | 2125         | 500        |
      | VA4     | 1500         | 500        |
    And total reputation is 6000
    And voting with id 0 in KycVoter is canceled.