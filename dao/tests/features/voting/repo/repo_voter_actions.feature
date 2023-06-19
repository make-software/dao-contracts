Feature: Repo Voter sets a value in the repository
  If voting passes, then the requested value is upated, if fails, the value remains the same.

Background:
  Given users
    | user    | is_va | REP balance |
    | Alice   | false | 0           |
    | Bob     | false | 0           | 
    | VA1     | true  | 1000        |
    | VA2     | true  | 1000        |
    | VA3     | true  | 1000        |
  And following configuration
      | key            | value |
      | PostJobDOSFee  | 10    |

Scenario: RepoVoter sets the value of PostJobDOSFee
  When RepoVoter voting with id 0 created by VA1 passes
    | voting_contract | stake | arg1               | arg2             | arg3   |
    | RepoVoter       | 100   | VariableRepository | PostJobDOSFee    | 12345  |
  Then value of PostJobDOSFee is 12345
  When RepoVoter voting with id 1 created by VA1 fails
    | voting_contract | stake | arg1               | arg2             | arg3   |
    | RepoVoter       | 100   | VariableRepository | PostJobDOSFee    | 10     |
  Then value of PostJobDOSFee is 12345

Scenario: RepoVoter sets the of PostJobDOSFee with activation time
  When RepoVoter voting with id 0 created by VA1 passes
    | voting_contract | stake | arg1               | arg2             | arg3   | arg4    |
    | RepoVoter       | 100   | VariableRepository | PostJobDOSFee    | 12345  | 14 days |
  Then value of PostJobDOSFee is 10
  # voting lasts 12 days
  When 3 days passed
  Then value of PostJobDOSFee is 12345
