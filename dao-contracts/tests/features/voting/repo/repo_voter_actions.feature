Feature: Repo Voter sets a value in the repository

Background:
  Given users
    | user    | is_va | REP balance | whitelisted_in  |
    | Alice   | false | 0           |                 |
    | Bob     | false | 0           | ReputationToken |
    | VA1     | true  | 1000        |                 |
    | VA2     | true  | 1000        |                 |
    | VA3     | true  | 1000        |                 |

Scenario: Voting passed, RepoVoter sets the value of PostJobDOSFee
  When VA1 starts voting with the following config
    | voting_contract | stake | arg1               | arg2             | arg3   |
    | RepoVoter       | 100   | VariableRepository | PostJobDOSFee    | 12345  |
  When voters vote in RepoVoter informal voting with id 0
    | user    | REP stake  | choice   | 
   #| VA1     | 100        | yes      | - automatically voted by the system
    | VA2     | 500        | yes      |
    | VA3     | 500        | yes      |
  And 5 days passed
  And informal voting with id 0 ends in RepoVoter contract
  And 2 days passed
  And voters vote in RepoVoter formal voting with id 0
    | user    | REP stake  | choice   | 
   #| VA1     | 100        | yes      | - automatically voted by the system
    | VA2     | 500        | yes      |
    | VA3     | 500        | yes      |
  And 5 days passed
  And formal voting with id 0 ends in RepoVoter contract
  Then value of PostJobDOSFee is 12345
