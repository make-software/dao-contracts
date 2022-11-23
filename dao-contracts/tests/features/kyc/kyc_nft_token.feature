Feature: KYC Token

  Scenario: KYC Token initial state.
    Then total supply is 0 tokens

  Rule: Only whitelisted user can mint a KYC Token.  
    Background:
      Given users
        | user   | is_whitelisted |
        | Alice  | false          |
        | Bob    | true           |
    Scenario Outline: If <minter> mints a KYC Token, the balance is updated
      When <minter> mints a KYC Token to any user.
      Then the user's balance is <balance>.

    Examples:
      | minter | balance |
      | Owner  | 1       |
      | Alice  | 0       |
      | Bob    | 1       |

  Rule: Owner can mint the first token to a user.
    Scenario: If Owner mints KYC Token to Bob, he's balance is updated.
      When Owner mints a KYC Token to Bob.
      Then the Bob's balance is 1.
      And Token with id 0 belongs to Bob.

  Rule: User can own only one KYC Token.
    Background:
      Given user Bob that owns a KYC Token.
    
    Scenario: If Owner mints the second KYC Token for Bob, he's balance remains the same.
      When Owner mints a KYC Token to Bob.
      Then the Bob's balance is 1.