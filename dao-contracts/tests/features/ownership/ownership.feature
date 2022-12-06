Feature: KYC Token ownership management
  Background:
    Given users
      | user    | whitelisted_in                                      |
      | Alice   |                                                     |
      | Bob     | KycToken,VaToken,VariableRepository,ReputationToken |

  Scenario Outline: Deploy a new instance
    Then Deployer is the owner of <contract> contract
    And Deployer is whitelisted in <contract> contract

    Examples: 
      | contract           |
      | KycToken           |
      | VaToken            |
      | VariableRepository |
      | ReputationToken    |
  
  Rule: Only the current owner can change ownership
    Scenario Outline: The current owner changes ownership
      When Deployer sets Alice as a new owner of <contract> contract
      Then Alice is the owner of <contract> contract
      And Alice is whitelisted in <contract> contract
      And Deployer is whitelisted in <contract> contract

      Examples: 
        | contract           |
        | KycToken           |
        | VaToken            |
        | VariableRepository |
        | ReputationToken    |

    Scenario Outline: A user out of the whitelist claims ownership
      When Alice sets Alice as a new owner of <contract> contract
      Then Deployer is the owner of <contract> contract

      Examples: 
        | contract           |
        | KycToken           |
        | VaToken            |
        | VariableRepository |
        | ReputationToken    |
    
    Scenario Outline: A whitelisted user claims ownership
      When Bob sets Bob as a new owner of <contract> contract
      Then Deployer is the owner of <contract> contract

      Examples: 
        | contract           |
        | KycToken           |
        | VaToken            |
        | VariableRepository |
        | ReputationToken    |
    
  Rule: Only the current owner can remove a user from the whitelist
    Scenario Outline: Owners removes from the whitelist
      When Deployer removes Bob from whitelist in <contract> contract
      Then Bob is not whitelisted in <contract> contract
      
      Examples: 
        | contract           |
        | KycToken           |
        | VaToken            |
        | VariableRepository |
        | ReputationToken    |

    Scenario Outline: Non-owner removes from the whitelist
      When Bob removes Bob from whitelist in <contract> contract
      Then Bob is whitelisted in <contract> contract

      Examples: 
        | contract           |
        | KycToken           |
        | VaToken            |
        | VariableRepository |
        | ReputationToken    |

  Rule: Only the current owner can remove a user from the whitelist
    Scenario Outline: Owner adds to the whitelist
      When Deployer adds Alice to whitelist in <contract> contract
      Then Alice is whitelisted in <contract> contract

      Examples: 
        | contract           |
        | KycToken           |
        | VaToken            |
        | VariableRepository |
        | ReputationToken    |

    Scenario Outline: A user adds to the whitelist
      When Bob adds Alive to whitelist in <contract> contract
      Then Alice is not whitelisted in <contract> contract

      Examples: 
        | contract           |
        | KycToken           |
        | VaToken            |
        | VariableRepository |
        | ReputationToken    |
  