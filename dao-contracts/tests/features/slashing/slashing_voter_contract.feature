Feature: Slashing in voter contract

  Background:
    Given following balances
      | account | REP balance | is_kyced | is_va |
      | VA1     | 1000        | true     | true  |
      | VA2     | 2000        | true     | true  |
      | VA3     | 2000        | true     | true  |
    When VA1 creates random voting in KycVoter with 500 stake
    And voters vote in KycVoter's informal voting with id 0
      | account | stake | vote | 
    # | VA1     | 500   | yes  | - automatically voted by the system
      | VA2     | 500   | yes  |
      | VA3     | 500   | yes  |

  Scenario: Voting creator gets slashed.
    When KycVoter slashes VA1 in voting with id 0
    Then balances are
      | account  | REP balance  | REP stake  |
      | VA1      | 1000         | 0          |
      | VA2      | 2000         | 0          |
      | VA3      | 2000         | 0          |
    And KycVoter's voting with id 0 is canceled.

  Scenario: Voting participan gets slashed.
    When KycVoter slashes VA2 in voting with id 0
    Then balances are
      | account  | REP balance  | REP stake  |
      | VA1      | 1000         | 500        |
      | VA2      | 2000         | 0          |
      | VA3      | 2000         | 500        |
