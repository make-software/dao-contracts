Feature: Staking
    Until voting is not finished, the tokens used to vote, are staked 
    
    Scenario Outline: Check staked balances
      Given users
        | user    | is_va        | REP balance |
        | Alice   | false        | 0           |
        | VA1     | true         | 1000        |
        | VA2     | true         | 1000        |
        | VA3     | true         | 1000        |
        | VA5     | true         | 1000        |
      And VA1 starts voting with the following config
        | voting_contract   | stake | arg1   | arg2   | arg3   |
        | <voting_contract> | 100   | <arg1> | <arg2> | <arg3> |
      When voters vote in <voting_contract> informal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
        | VA3     | 200        | yes    |
      Then users balances are
        | account | REP balance | REP stake |
        | VA1     | 1000        | 100       |
        | VA2     | 1000        | 500       |
        | VA3     | 1000        | 200       |
      Examples:
        | voting_contract  | arg1               | arg2             | arg3  |
        | KycVoter         | Alice              |                  |       |      
        | Admin            | ReputationToken    | add_to_whitelist | Alice |
        | SlashingVoter    | VA5                | 1                |       |
        | RepoVoter        | VariableRepository | PostJobDOSFee    | 1     | 
        | SimpleVoter      |                    |                  |       |
        | ReputationVoter  | Alice              | mint             | 100   |
