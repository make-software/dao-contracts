Feature: Kyc Voter
  Background:
    Given users
      | user    | is_va | REP balance |
      | Alice   | false | 0           |
      | VA1     | true  | 1000        |
      | VA2     | true  | 1000        |
      | VA3     | true  | 1000        |
      | VA4     | true  | 1000        |

  Scenario: Quorum reached, voting passed
    When KycVoter voting with id 0 created by VA1 passes
      | voting_contract | stake | arg1  |
      | KycVoter        | 100   | Alice |
    Then Alice is kyced

  Scenario: Quorum reached, voting rejected
    When KycVoter voting with id 0 created by VA1 fails
      | voting_contract | stake | arg1  |
      | KycVoter        | 100   | Alice |
    Then Alice is not kyced
     