Feature: Creating voting for a va
    Background:
      Given users
        | user    | is_va        | is_kyced | REP balance |
        | Alice   | false        | true     | 0           |
        | VA1     | true         | true     | 1000        |
        | VA2     | true         | true     | 1000        |

    Scenario: Cannot create a voting for a user who already is a va
      When VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |
      Then informal voting with id 0 in KycVoter contract does not start
    
    
