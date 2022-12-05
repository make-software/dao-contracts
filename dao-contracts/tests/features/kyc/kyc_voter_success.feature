Feature: Kyc Voter
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
      And Owner added VA1 to whitelist in KycVoter contract
      And VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |

    Scenario: Quorum reached, voting passed
      When voters vote in KycVoter's informal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 100        | in favor |
        | VA3     | 200        | in favor |
        | VA4     | 250        | against  |
      And 5 days passed
      And informal voting with id 0 ends in KycVoter contract
      And 1 days passed
      And voters vote in KycVoter's formal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 100        | in favor |
        | VA3     | 200        | in favor |
        | VA4     | 250        | against  |
      And 5 days passed
      And formal voting with id 0 ends in KycVoter contract
      Then users balances are
        | account | REP balance  | REP stake  |
        | VA1     | 1062         | 0          |
        | VA2     | 1062         | 0          |
        | VA3     | 1125         | 0          |
        | VA4     | 750          | 0          |
        | VA5     | 1000         | 0          |
        | VA6     | 1000         | 0          |
      And Alice is kyced
     