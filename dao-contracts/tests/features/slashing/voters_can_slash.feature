Feature: Slashing in voter contract

  Background:
    Given following balances
      | account | REP balance | is_kyced | is_va |
      | VA1     | 1000        | true     | true  |
      | VA2     | 2000        | true     | true  |
      | VA3     | 2000        | true     | true  |
    
    Scenario Outline: Voting creator gets slashed.
      When VA1 creates test voting in <contract> with 500 stake
      And voters vote in <contract> informal voting with id 0
        | account | stake | vote | 
      # | VA1     | 500   | yes  | - automatically voted by the system
        | VA2     | 500   | yes  |
        | VA3     | 500   | yes  |
      When <contract> slashes VA1 in voting with id 0
      Then balances are
        | account  | REP balance  | REP stake  |
        | VA1      | 1000         | 0          |
        | VA2      | 2000         | 0          |
        | VA3      | 2000         | 0          |
    # And <contract> voting with id 0 is canceled.

      Examples:
        | contract        |
        | KycVoter        |
        | RepoVoter       |
        | ReputationVoter |
        | SimpleVoter     |
        | SlashingVoter   |
        | Admin           |

    Scenario Outline: Voting participan gets slashed.
      When VA1 creates test voting in <contract> with 500 stake
      Then balances are
        | account  | REP balance  | REP stake  |
        | VA1      | 1000         | 500        |
        | VA2      | 2000         | 0          |
        | VA3      | 2000         | 0          |
      When voters vote in <contract> informal voting with id 0
        | account | stake | vote | 
      # | VA1     | 500   | yes  | - automatically voted by the system
        | VA2     | 500   | yes  |
        | VA3     | 500   | yes  |
      Then balances are
        | account  | REP balance  | REP stake  |
        | VA1      | 1000         | 500        |
        | VA2      | 2000         | 500        |
        | VA3      | 2000         | 500        |
      When <contract> slashes VA2 in voting with id 0
      Then balances are
        | account  | REP balance  | REP stake  |
        | VA1      | 1000         | 500        |
        | VA2      | 2000         | 0          |
        | VA3      | 2000         | 500        |

      Examples:
        | contract        |
        | KycVoter        |
        | RepoVoter       |
        | ReputationVoter |
        | SimpleVoter     |
        | SlashingVoter   |
        | Admin           |
