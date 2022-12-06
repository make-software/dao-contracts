Feature: VA Token events emission
  Scenario: Mint and Burn
    When Owner mints a VA Token to Bob
    And Owner burns Bob's VA token
    Then VaToken contract emits events
      | event            | arg1     | arg2      | arg3 |
      | Transfer         |          | Bob       | 0    |
      | Approval         | Bob      | Owner     | 0    |
      | Transfer         | Bob      |           | 0    |
