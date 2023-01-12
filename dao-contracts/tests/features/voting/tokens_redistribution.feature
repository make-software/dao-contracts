Feature: Tokens redistribution
    Tests tokens redistribution in non-BidEscrow voting.
    Background:
      Given users
        | user    | is_va | REP balance |
        | Alice   | false | 0           |
        | VA1     | true  | 1000        |
        | VA2     | true  | 1000        |
        | VA3     | true  | 1000        |
        | VA4     | true  | 1000        |
        | VA5     | true  | 1000        |
        | VA6     | true  | 1000        |

    Scenario Outline: Informal voting quorum not reached
      When VA1 starts voting with the following config
        | voting_contract    | stake | arg1    | arg2    | arg3   | arg4   |
        | <voting_contract>  | 100   | <arg1>  | <arg2>  | <arg3> | <arg4> |
      And voters vote in <voting_contract> informal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 250        | no     |
      And 5 days passed
      And informal voting with id 0 ends in <voting_contract> contract
      Then users balances are
        | account | REP balance  |
        | VA1     | 1000         |
        | VA2     | 1000         |
        | VA3     | 1000         |
        | VA4     | 1000         |
        | VA5     | 1000         |
        | VA6     | 1000         |
      And formal voting with id 0 in <voting_contract> contract does not start
      Examples:
        | voting_contract  | arg1               | arg2             | arg3  | arg4 | arg5 |
        | KycVoter         | Alice              |                  |       |      |      |      
        | Admin            | ReputationToken    | add_to_whitelist | Alice |      |      |
        | SlashingVoter    | Bob                | 1                |       |      |      |
        | RepoVoter        | VariableRepository | PostJobDOSFee    | 1     |      |      | 
        | SimpleVoter      |                    |                  |       |      |      |
        | ReputationVoter  | Alice              | mint             | 100   |      |      |

    Scenario Outline: Formal voting quorum not reached
      When VA1 starts voting with the following config
        | voting_contract    | stake | arg1    | arg2    | arg3   | arg4   |
        | <voting_contract>  | 100   | <arg1>  | <arg2>  | <arg3> | <arg4> |
      And voters vote in <voting_contract> informal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
        | VA3     | 200        | yes    |
        | VA4     | 250        | no     |
      And 5 days passed
      And informal voting with id 0 ends in <voting_contract> contract
      And 2 days passed
      And voters vote in <voting_contract> formal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
      And 5 days passed
      And formal voting with id 0 ends in <voting_contract> contract
      Then users balances are
        | account | REP balance  |
        | VA1     | 1000         |
        | VA2     | 1000         |
        | VA3     | 1000         |
        | VA4     | 1000         |
      Examples:
        | voting_contract  | arg1               | arg2             | arg3  | arg4 | arg5 |
        | KycVoter         | Alice              |                  |       |      |      |      
        | Admin            | ReputationToken    | add_to_whitelist | Alice |      |      |
        | SlashingVoter    | Bob                | 1                |       |      |      |
        | RepoVoter        | VariableRepository | PostJobDOSFee    | 1     |      |      | 
        | SimpleVoter      |                    |                  |       |      |      |
        | ReputationVoter  | Alice              | mint             | 100   |      |      |

    Scenario Outline: Voting passed
      When VA1 starts voting with the following config
        | voting_contract    | stake | arg1    | arg2    | arg3   | arg4   |
        | <voting_contract>  | 100   | <arg1>  | <arg2>  | <arg3> | <arg4> |
      And voters vote in <voting_contract> informal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
        | VA3     | 200        | yes    |
        | VA4     | 250        | no     |
      And 5 days passed
      And informal voting with id 0 ends in <voting_contract> contract
      And 2 days passed
      And voters vote in <voting_contract> formal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
        | VA3     | 200        | yes    |
        | VA4     | 250        | no     |
      And 5 days passed
      And formal voting with id 0 ends in <voting_contract> contract
      Then users balances are
        | account | REP balance  |
        | VA1     | 1031.25      |
        | VA2     | 1156.25      |
        | VA3     | 1062.5       |
        | VA4     | 750          |
     Examples:
        | voting_contract  | arg1               | arg2             | arg3  | arg4 | arg5 |
        | KycVoter         | Alice              |                  |       |      |      |      
        | Admin            | ReputationToken    | add_to_whitelist | Alice |      |      |
        | SlashingVoter    | Bob                | 1                |       |      |      |
        | RepoVoter        | VariableRepository | PostJobDOSFee    | 1     |      |      | 
        | SimpleVoter      |                    |                  |       |      |      |
        | ReputationVoter  | Alice              | mint             | 100   |      |      |

    Scenario Outline: Voting rejected
      When VA1 starts voting with the following config
        | voting_contract    | stake | arg1    | arg2    | arg3   | arg4   |
        | <voting_contract>  | 100   | <arg1>  | <arg2>  | <arg3> | <arg4> |
      And voters vote in <voting_contract> informal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | no     |
        | VA3     | 200        | no     |
        | VA4     | 250        | yes    |
      And 5 days passed
      And informal voting with id 0 ends in <voting_contract> contract
      And 2 days passed
      And voters vote in <voting_contract> formal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | no     |
        | VA3     | 200        | no     |
        | VA4     | 250        | yes    |
      And 5 days passed
      And formal voting with id 0 ends in <voting_contract> contract
      Then users balances are
        | account | REP balance  |
        | VA1     | 900          |
        | VA2     | 1250         |
        | VA3     | 1100         |
        | VA4     | 750          |
     Examples:
        | voting_contract  | arg1               | arg2             | arg3  | arg4 | arg5 |
        | KycVoter         | Alice              |                  |       |      |      |      
        | Admin            | ReputationToken    | add_to_whitelist | Alice |      |      |
        | SlashingVoter    | Bob                | 1                |       |      |      |
        | RepoVoter        | VariableRepository | PostJobDOSFee    | 1     |      |      | 
        | SimpleVoter      |                    |                  |       |      |      |
        | ReputationVoter  | Alice              | mint             | 100   |      |      |
