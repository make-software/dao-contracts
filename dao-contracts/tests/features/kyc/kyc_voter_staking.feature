Feature: Kyc Voter - staking
    Until voting is not finished, the tokens used to vote, are staked 
    Background:
      Given users
        | user    | is_va        | REP balance |
        | Alice   | false        | 0           |
        | VA1     | true         | 1000        |
        | VA2     | true         | 1000        |
        | VA3     | true         | 1000        |
        | VA4     | true         | 1000        |
        | VA5     | true         | 1000        |
        | VA6     | true         | 1000        |
      And VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |

    Scenario: Check staked balances
       When voters vote in KycVoter informal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 500        | in favor |
        | VA3     | 200        | in favor |
      Then users balances are
        | account | REP balance | REP stake |
        | VA1     | 1000        | 100       |
        | VA2     | 1000        | 500       |
        | VA3     | 1000        | 200       |