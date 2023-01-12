Feature: Admin manages other contracts ownership
  Admin contract is capable of add/remove to whitelist and the current owner of contract.

Background:
      Given users
        | user    | is_va | REP balance | whitelisted_in  |
        | Alice   | false | 0           |                 |
        | Bob     | false | 0           | ReputationToken |
        | VA1     | true  | 1000        |                 |
        | VA2     | true  | 1000        |                 |
        | VA3     | true  | 1000        |                 |

Scenario Outline: Voting passed, action applied
    When VA1 starts voting with the following config
        | voting_contract | stake | arg1            | arg2                | arg3      |
        | Admin           | 500   | ReputationToken | <action>            | <subject> |
    When voters vote in Admin informal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 500        | in favor |
        | VA3     | 500        | in favor |
    And 5 days passed
    And informal voting with id 0 ends in Admin contract
    And 2 days passed
    And voters vote in Admin formal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 500        | in favor |
        | VA3     | 500        | in favor |
    And 5 days passed
    And formal voting with id 0 ends in Admin contract
    Then <subject> <result> ReputationToken contract

    Examples:
        | action                | subject | result                 |
        | add_to_whitelist      | Alice   | is whitelisted in      |
        | remove_from_whitelist | Bob     | is not whitelisted in  |
        | change_ownership      | Bob     | is the owner of        |
