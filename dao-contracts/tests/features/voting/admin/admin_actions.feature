Feature: Admin Contract manages other contracts ownership
  Admin contract is capable of add/remove to whitelist and change the current owner of contract.

Background:
  Given users
    | user    | is_va | REP balance | whitelisted_in  |
    | Alice   | false | 0           |                 |
    | Bob     | false | 0           | ReputationToken |
    | VA1     | true  | 1000        |                 |
    | VA2     | true  | 1000        |                 |
    | VA3     | true  | 1000        |                 |

Scenario Outline: Voting passed, action applied
  When Admin voting with id 0 created by VA1 passes
    | voting_contract | stake | arg1            | arg2     | arg3      |
    | Admin           | 500   | ReputationToken | <action> | <subject> |
  Then <subject> <result>
  Examples:
    | action                | subject | result                                         |
    | add_to_whitelist      | Alice   | is whitelisted in ReputationToken contract     |
    | remove_from_whitelist | Bob     | is not whitelisted in ReputationToken contract |
    | change_ownership      | Bob     | is the owner of ReputationToken contract       |
