Feature: Cancelling finished voting
    If a month passes and the voting is not finished, the voting can be cancelled

  Scenario: Cancel finished voting over admitting external worker to be a va
    Given following balances
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | MultisigWallet   | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | ExternalWorker   | 500          | 0            | 0          | true     | false |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And following configuration
      | key                                    | value         |
      | TimeBetweenInformalAndFormalVoting     | 0             |
      | VotingStartAfterJobSubmission          | 0             |
    When JobPoster posted a JobOffer with expected timeframe of 14 days, maximum budget of 1000 CSPR and 400 CSPR DOS Fee
    And InternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 100 REP stake
    And 8 days passed
    And ExternalWorker posted the Bid for JobOffer 0 with proposed timeframe of 7 days and 500 CSPR price and 500 CSPR stake with onboarding
    And JobPoster picked the Bid of ExternalWorker
    When ExternalWorker submits the JobProof of Job 0
    And voters vote in BidEscrow informal voting with id 0
      | account          | REP stake | choice |
     #| ExternalWorker   | 50        | Yes    |- automatically voted by the system - 500CSPR converted to 50 Reputation
      | VA1              | 500       | Yes    |
      | VA2              | 500       | No     |
    When 6 days passed
    And informal voting with id 0 ends in BidEscrow contract
    When voters vote in BidEscrow formal voting with id 0
      | account          | REP stake | choice |
     #| ExternalWorker   | 50        | Yes    | - automatically voted by the system
      | VA1              | 500       | Yes    |
      | VA2              | 500       | No     |
    # Now voting time has passed, but it hasn't been finished
    And 6 days passed
    # We let pass time
    And 20 days passed
    And VA1 cannot cancel voting in BidEscrow contract with id 0
    # Now we let the time pass some more
    And 11 days passed
    And VA1 cancels voting in BidEscrow contract with id 0
    Then balances are
    # Exactly the same
      | account          | CSPR balance | REP balance  | REP stake  | is_kyced | is_va |
      | BidEscrow        | 0            | 0            | 0          | false    | false |
      | MultisigWallet   | 0            | 0            | 0          | false    | false |
      | JobPoster        | 1000         | 0            | 0          | true     | false |
      | InternalWorker   | 0            | 1000         | 0          | true     | true  |
      | ExternalWorker   | 500          | 0            | 0          | true     | false |
      | VA1              | 0            | 1000         | 0          | true     | true  |
      | VA2              | 0            | 1000         | 0          | true     | true  |
    And total reputation is 3000
    And ExternalWorker is not a VA

  Scenario: Cancel finished voting for Onboarding request
    Given following balances
      | account          | CSPR balance | REP balance  | is_kyced | is_va |
      | Onboarding       | 0            | 0            | false    | false |
      | MultisigWallet   | 0            | 0            | false    | false |
      | Bob              | 1000         | 0            | true     | false |
      | VA1              | 0            | 1000         | true     | true  |
      | VA2              | 0            | 1000         | true     | true  |
    And following configuration
      | key                                    | value |
      | TimeBetweenInformalAndFormalVoting     | 0     |
      | VotingStartAfterJobSubmission          | 0     |
    When Bob submits an onboarding request with the stake of 1000 CSPR
    When voters vote in Onboarding informal voting with id 0
      | user    | REP stake  | choice |
     #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
      | VA1     | 500        | yes    |
      | VA2     | 500        | yes    |
    When 6 days passed
    And informal voting with id 0 ends in Onboarding contract
    When voters vote in Onboarding formal voting with id 0
      | user    | REP stake  | choice |
     #| Bob     | 100        | yes    | - automatically voted by the system - 1000CSPR converted to 100 Reputation
      | VA1     | 500        | yes    |
      | VA2     | 500        | yes    |
    And 6 days passed
    And 31 days passed
    And VA1 cancels voting in Onboarding contract with id 0
    Then total reputation is 2000
    And Bob is not a VA
    And balances are
      | account          | CSPR balance | REP balance  | is_kyced | is_va |
      | Onboarding       | 0            | 0            | false    | false |
      | MultisigWallet   | 0            | 0            | false    | false |
      | Bob              | 1000         | 0            | true     | false |
      | VA1              | 0            | 1000         | true     | true  |
      | VA2              | 0            | 1000         | true     | true  |
    And total reputation is 2000

    Scenario Outline: Cancel voting in regular voting
      Given users
        | user    | is_va        | REP balance |
        | VA1     | true         | 1000        |
        | VA2     | true         | 1000        |
        | VA3     | true         | 1000        |
        | VA5     | true         | 1000        |
      And VA1 starts voting with the following config
        | voting_contract   | stake | arg1   | arg2   | arg3   |
        | <voting_contract> | 100   | <arg1> | <arg2> | <arg3> |
      When voters vote in <voting_contract> informal voting with id 0
        | user    | REP stake  | choice |
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
        | VA3     | 200        | yes    |
      And 5 days passed
      And informal voting with id 0 ends in <voting_contract> contract
      And 2 days passed
      And voters vote in <voting_contract> formal voting with id 0
        | user    | REP stake  | choice |
       #| VA1     | 100        | yes    | - automatically voted by the system
        | VA2     | 500        | yes    |
      And 5 days passed
      # Now voting time has passed, but it hasn't been finished
      And 31 days passed
      # Now voting can be cancelled
      When VA1 cancels voting in <voting_contract> contract with id 0
      Then users balances are
      # Everything is returned to the users
        | account | REP balance | REP stake |
        | VA1     | 1000        | 0         |
        | VA2     | 1000        | 0         |
        | VA3     | 1000        | 0         |

      Examples:
        | voting_contract  | arg1               | arg2             | arg3  |
        | KycVoter         | Alice              |                  |       |
        | Admin            | ReputationToken    | add_to_whitelist | Alice |
        | SlashingVoter    | VA5                | 1                |       |
        | RepoVoter        | VariableRepository | PostJobDOSFee    | 1     |
        | SimpleVoter      |                    |                  |       |
        | ReputationVoter  | Alice              | mint             | 100   |