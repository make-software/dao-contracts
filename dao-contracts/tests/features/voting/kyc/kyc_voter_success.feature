Feature: Kyc Voter
    Background:
      Given users
        | user    | is_va | REP balance |
        | Alice   | false | 0           |
        | VA1     | true  | 1000        |
        | VA2     | true  | 1000        |
        | VA3     | true  | 1000        |
        | VA4     | true  | 1000        |
      And VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |

    Scenario: Quorum reached, voting passed
      When voters vote in KycVoter informal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
        | VA3     | 200        | yes    |
        | VA4     | 250        | no     |
      And 5 days passed
      And informal voting with id 0 ends in KycVoter contract
      And 2 days passed
      And voters vote in KycVoter formal voting with id 0
        | user    | REP stake  | choice | 
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
        | VA3     | 200        | yes    |
        | VA4     | 250        | no     |
      And 5 days passed
      And formal voting with id 0 ends in KycVoter contract
      Then Alice is kyced
     