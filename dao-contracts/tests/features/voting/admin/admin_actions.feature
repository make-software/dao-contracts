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
  When VA1 starts voting with the following config
    | voting_contract | stake | arg1            | arg2     | arg3      |
    | Admin           | 500   | ReputationToken | <action> | <subject> |
  When voters vote in Admin informal voting with id 0
    | user    | REP stake  | choice   | 
   #| VA1     | 500        | yes      | - automatically voted by the system
    | VA2     | 500        | yes      |
    | VA3     | 500        | yes      |
  And 5 days passed
  And informal voting with id 0 ends in Admin contract
  And 2 days passed
  And voters vote in Admin formal voting with id 0
    | user    | REP stake  | choice   | 
   #| VA1     | 500        | yes      | - automatically voted by the system
    | VA2     | 500        | yes      |
    | VA3     | 500        | yes      |
  And 5 days passed
  And formal voting with id 0 ends in Admin contract
  Then <subject> <result>

  Examples:
    | action                | subject | result                                         |
    | add_to_whitelist      | Alice   | is whitelisted in ReputationToken contract     |
    | remove_from_whitelist | Bob     | is not whitelisted in ReputationToken contract |
    | change_ownership      | Bob     | is the owner of ReputationToken contract       |
