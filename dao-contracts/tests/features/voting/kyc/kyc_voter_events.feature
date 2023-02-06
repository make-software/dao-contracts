Feature: Kyc Voter events
    Scenario: Events emitted
      Given users
        | user    | is_va | REP balance |
        | Alice   | false | 0           |
        | VA1     | true  | 1000        |
        | VA2     | true  | 1000        |
        | VA3     | true  | 1000        |
        | VA4     | true  | 1000        |
      When KycVoter voting with id 0 created by VA1 passes
        | voting_contract | stake | arg1  |
        | KycVoter        | 100   | Alice |
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
