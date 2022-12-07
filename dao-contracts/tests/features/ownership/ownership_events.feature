Feature: Ownership and whitelisting events

  Scenario Outline: Ownership
    When Owner sets Alice as a new owner of <contract> contract
    And Alice removes Owner from whitelist in <contract> contract
    Then <contract> contract emits events
      | event                | arg1  |
      | OwnerChanged         | Alice |
      | AddedToWhitelist     | Alice |
      | RemovedFromWhitelist | Owner |
    
    Examples: 
      | contract           |
      | KycToken           |
      | VaToken            |
      | VariableRepository |
      | ReputationToken    |

  Scenario Outline: Whitelisting
    When Owner adds Alice to whitelist in <contract> contract
    And Owner removes Alice from whitelist in <contract> contract
    Then <contract> contract emits events
      | event                | arg1  |
      | AddedToWhitelist     | Alice |
      | RemovedFromWhitelist | Alice |

    Examples: 
      | contract           |
      | KycToken           |
      | VaToken            |
      | VariableRepository |
      | ReputationToken    |
