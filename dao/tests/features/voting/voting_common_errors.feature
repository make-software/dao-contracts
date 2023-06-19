Feature: Voting errors
    Voters cast invalid votes in non-BidEscrow voting.
    Background:
      Given users
        | user    | is_va        | REP balance |
        | Alice   | false        | 0           |
        | VA1     | true         | 1000        |
        | VA2     | true         | 1000        |
        | VA3     | true         | 1000        |
        | VA4     | true         | 1000        |
        | VA5     | true         | 1000        |

    Scenario Outline: Invalid votes
      When VA1 starts voting with the following config
        | voting_contract   | stake | arg1   | arg2   | arg3   |
        | <voting_contract> | 100   | <arg1> | <arg2> | <arg3> |
      And voters vote in <voting_contract> informal voting with id 0
        | user    | REP stake  | choice  | 
       #| VA1     | 100        | yes     | - automatically voted by the system
        | VA2     | 200        | yes     |
      Then votes in <voting_contract> informal voting with id 0 fail
        | user    | REP stake  | choice   | result              |
        | VA1     | 100        | against  | CannotVoteTwice     |
        | VA3     | 0          | against  | ZeroStake           |
        | VA4     | 1001       | against  | InsufficientBalance |
      And users balances are
        | account | REP balance  | REP stake  |
        | VA1     | 1000         | 100        |
        | VA2     | 1000         | 200        |
    
    Examples:
        | voting_contract  | arg1               | arg2             | arg3  |
        | KycVoter         | Alice              |                  |       |      
        | Admin            | ReputationToken    | add_to_whitelist | Alice |
        | SlashingVoter    | VA5                | 1                |       |
        | RepoVoter        | VariableRepository | PostJobDOSFee    | 1     | 
        | SimpleVoter      |                    |                  |       |
        | ReputationVoter  | Alice              | mint             | 100   |
