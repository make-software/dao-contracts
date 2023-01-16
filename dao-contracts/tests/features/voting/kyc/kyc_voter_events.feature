Feature: Kyc Voter events
    Scenario: Events emitted
      Given users
        | user    | is_va | REP balance |
        | Alice   | false | 0           |
        | VA1     | true  | 1000        |
        | VA2     | true  | 1000        |
        | VA3     | true  | 1000        |
        | VA4     | true  | 1000        |
      When VA1 starts voting with the following config
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |
      And voters vote in KycVoter informal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 500        | in favor |
        | VA3     | 200        | in favor |
        | VA4     | 100        | against  |
      And 5 days passed
      And informal voting with id 0 ends in KycVoter contract
      And 2 days passed
      And voters vote in KycVoter formal voting with id 0
        | user    | REP stake  | choice   | 
        | VA2     | 500        | in favor |
        | VA3     | 200        | in favor |
        | VA4     | 300        | against  |
      # Then KycVoter contract emits events
      #   | event                 | arg1               | arg2            | arg3     | arg4  | arg5   | arg6   | arg7   | arg8   |
      #   | VotingContractCreated | VariableRepository | ReputationToken | KycVoter |       |        |        |        |        |
      #   | VotingCreated         | VA1                | 0               | 0        |       | 2      | 432000 | 2      | 432000 |
      #   | BallotCast            | VA1                | 0               | in favor | 100   |        |        |        |        |
      #   | BallotCast            | VA2                | 0               | in favor | 500   |        |        |        |        |
      #   | BallotCast            | VA3                | 0               | in favor | 200   |        |        |        |        |
      #   | BallotCast            | VA4                | 0               | against  | 100   |        |        |        |        |
      #   | VotingCreated         | VA1                | 1               | 0        | 1     | 2      | 432000 | 2      | 432000 |
      #   | BallotCast            | VA1                | 1               | in favor | 100   |        |        |        |        |
      #   | BallotCast            | VA2                | 1               | in favor | 500   |        |        |        |        |
      #   | BallotCast            | VA3                | 1               | in favor | 200   |        |        |        |        |
      #   | BallotCast            | VA4                | 1               | against  | 300   |        |        |        |        |
