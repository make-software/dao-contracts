Feature: KYC Token events emission
  Scenario: Deploy emits 2 events
    Then KycToken contract emits events
      | event            | arg1     |
      | OwnerChanged     | Deployer |
      | AddedToWhitelist | Deployer |

  Scenario: Mint and Burn
    When Owner mints a KYC Token to Bob
    And Owner burns Bob's token
    Then KycToken contract emits events
      | event            | arg1     | arg2      | arg3 |
      | Transfer         |          | Bob       | 0    |
      | Approval         | Bob      | Owner     | 0    |
      | Transfer         | Bob      |           | 0    |

  Scenario: Ownership
    When Owner sets Alice as a new owner of KycToken contract
    And Alice removes Owner from whitelist in KycToken contract
    Then KycToken contract emits events
      | event                | arg1     |
      | OwnerChanged         | Alice    |
      | AddedToWhitelist     | Alice    |
      | RemovedFromWhitelist | Owner    |

  Scenario: Whitelisting
    When Owner adds Alice to whitelist in KycToken contract
    And Owner removes Alice from whitelist in KycToken contract
    Then KycToken contract emits events
      | event                | arg1     |
      | AddedToWhitelist     | Alice    |
      | RemovedFromWhitelist | Alice    |
