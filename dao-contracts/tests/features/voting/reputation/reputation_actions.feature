Feature: Test Reputation Voter Contract actions
  Reputation Voter contract is capable of mint/burn tokens for an account.

Scenario Outline: Voting passed - perform action
  Given users
    | user    | is_va | REP balance | whitelisted_in  |
    | Alice   | false | 1000        |                 |
    | VA1     | true  | 1000        |                 |
    | VA2     | true  | 1000        |                 |
    | VA3     | true  | 1000        |                 |
  When ReputationVoter voting with id 0 created by VA1 passes
    | voting_contract | stake | arg1  | arg2     | arg3     |
    | ReputationVoter | 500   | Alice | <action> | <amount> |
  Then balance of Alice is <expected_value>
  Examples:
    | action | amount  | expected_value |
    | mint   | 100     | 1100           |
    | burn   | 200     | 800            |
