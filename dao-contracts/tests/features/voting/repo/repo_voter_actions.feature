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
  When RepoVoter voting with id 0 created by VA1 passes
    | voting_contract | stake | arg1               | arg2             | arg3   |
    | RepoVoter       | 100   | VariableRepository | PostJobDOSFee    | 12345  |
  Then value of PostJobDOSFee is 12345
