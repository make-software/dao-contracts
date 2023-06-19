Feature: Voting id generation
  The system has a single source 

  Scenario: Each voting generates a next id
    Given users
      | user             | is_va   | is_kyced | REP balance | CSPR balance |
      | VA1              | true    | true     | 1000        | 1000         |
      | VA2              | true    | true     | 1000        | 1000         |
      | Bob              | false   | true     | 0           | 1000         |
      | InternalWorker   | true    | true     | 1000        | 0            |
      | JobPoster        | false   | true     | 0           | 1000         |
    And following configuration
      | key                                    | value |
      | VotingStartAfterJobSubmission          | 0     |
    When VA1 starts voting with the following config
      | voting_contract | stake |
      | SimpleVoter     | 100   |
    Then voting with id 0 in SimpleVoter contract starts
    When VA1 starts voting with the following config
      | voting_contract | stake | arg1            | arg2             | arg3  |
      | Admin           | 100   | ReputationToken | add_to_whitelist | Alice |
    Then voting with id 1 in Admin contract starts
    When VA1 starts voting with the following config
      | voting_contract | stake | arg1               | arg2          | arg3  |
      | RepoVoter       | 100   | VariableRepository | PostJobDOSFee | 1     |
    Then voting with id 2 in RepoVoter contract starts
    When VA1 starts voting with the following config
      | voting_contract | stake | arg1 | arg2 |
      | SlashingVoter   | 100   | VA2  | 1    |
    Then voting with id 3 in SlashingVoter contract starts
    When VA1 starts voting with the following config
      | voting_contract | stake | arg1  |
      | KycVoter        | 100   | Alice |
    Then voting with id 4 in KycVoter contract starts
    When VA1 starts voting with the following config
      | voting_contract | stake | arg1  | arg2 | arg3 |
      | ReputationVoter | 100   | Alice | mint | 100  |
    Then voting with id 5 in ReputationVoter contract starts
    When Bob submits an onboarding request with the stake of 1000 CSPR
    Then voting with id 6 in Onboarding contract starts
    # BidEscrow setup 
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 CSPR stake with onboarding
    And JobPoster picked the Bid of InternalWorker
    And InternalWorker submits the JobProof of Job 0
    Then voting with id 7 in BidEscrow contract starts

    