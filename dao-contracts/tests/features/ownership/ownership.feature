Feature: KYC Token ownership management
  Scenario Outline: Deploy a new instance
    Then Deployer is the owner of <contract> contract
    And Deployer is whitelisted in <contract> contract

    Examples: 
      | contract           |
      | Admin              |
      | BidEscrow          |
      | KycToken           |
      | KycVoter           |
      | Onboarding         |
      | RepoVoter          |
      | ReputationToken    |
      | ReputationVoter    |
      | SimpleVoter        |
      | SlashingVoter      |
      | VaToken            |
      | VariableRepository |
  
  Rule: Only the current owner can change ownership
    Scenario Outline: The current owner changes ownership
      When Deployer sets Alice as a new owner of <contract> contract
      Then Alice is the owner of <contract> contract
      And Alice is whitelisted in <contract> contract
      And Deployer is whitelisted in <contract> contract

      Examples: 
        | contract           |
        | Admin              |
        | BidEscrow          |
        | KycToken           |
        | KycVoter           |
        | Onboarding         |
        | RepoVoter          |
        | ReputationToken    |
        | ReputationVoter    |
        | SimpleVoter        |
        | SlashingVoter      |
        | VaToken            |
        | VariableRepository |

    Scenario Outline: A user out of the whitelist claims ownership
      When Alice sets Alice as a new owner of <contract> contract
      Then Deployer is the owner of <contract> contract

      Examples: 
        | contract           |
        | Admin              |
        | BidEscrow          |
        | KycToken           |
        | KycVoter           |
        | Onboarding         |
        | RepoVoter          |
        | ReputationToken    |
        | ReputationVoter    |
        | SimpleVoter        |
        | SlashingVoter      |
        | VaToken            |
        | VariableRepository |
    
    Scenario Outline: A whitelisted user claims ownership
      When Deployer adds Bob to whitelist in <contract> contract
      And Bob sets Bob as a new owner of <contract> contract
      Then Deployer is the owner of <contract> contract

      Examples: 
        | contract           |
        | Admin              |
        | BidEscrow          |
        | KycToken           |
        | KycVoter           |
        | Onboarding         |
        | RepoVoter          |
        | ReputationToken    |
        | ReputationVoter    |
        | SimpleVoter        |
        | SlashingVoter      |
        | VaToken            |
      | VariableRepository |
    
  Rule: Only the current owner can remove a user from the whitelist
    Scenario Outline: Owners removes from the whitelist
      When Deployer adds Bob to whitelist in <contract> contract
      And Deployer removes Bob from whitelist in <contract> contract
      Then Bob is not whitelisted in <contract> contract
      
      Examples: 
        | contract           |
        | Admin              |
        | BidEscrow          |
        | KycToken           |
        | KycVoter           |
        | Onboarding         |
        | RepoVoter          |
        | ReputationToken    |
        | ReputationVoter    |
        | SimpleVoter        |
        | SlashingVoter      |
        | VaToken            |
        | VariableRepository |

    Scenario Outline: Non-owner removes from the whitelist
      When Deployer adds Bob to whitelist in <contract> contract
      And Bob removes Bob from whitelist in <contract> contract
      Then Bob is whitelisted in <contract> contract

      Examples: 
        | contract           |
        | Admin              |
        | BidEscrow          |
        | KycToken           |
        | KycVoter           |
        | Onboarding         |
        | RepoVoter          |
        | ReputationToken    |
        | ReputationVoter    |
        | SimpleVoter        |
        | SlashingVoter      |
        | VaToken            |
        | VariableRepository |

  Rule: Only the current owner can remove a user from the whitelist
    Scenario Outline: Owner adds to the whitelist
      When Deployer adds Alice to whitelist in <contract> contract
      Then Alice is whitelisted in <contract> contract

      Examples: 
        | contract           |
        | Admin              |
        | BidEscrow          |
        | KycToken           |
        | KycVoter           |
        | Onboarding         |
        | RepoVoter          |
        | ReputationToken    |
        | ReputationVoter    |
        | SimpleVoter        |
        | SlashingVoter      |
        | VaToken            |
        | VariableRepository |

    Scenario Outline: A user adds to the whitelist
      When Deployer adds Bob to whitelist in <contract> contract
      And Bob adds Alice to whitelist in <contract> contract
      Then Alice is not whitelisted in <contract> contract

      Examples: 
        | contract           |
        | Admin              |
        | BidEscrow          |
        | KycToken           |
        | KycVoter           |
        | Onboarding         |
        | RepoVoter          |
        | ReputationToken    |
        | ReputationVoter    |
        | SimpleVoter        |
        | SlashingVoter      |
        | VaToken            |
        | VariableRepository |
  