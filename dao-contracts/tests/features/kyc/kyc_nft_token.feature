Feature: KYC Token

  Scenario: KYC Token initial state.
    Then total supply is 0 tokens

  Rule: Only whitelisted user can mint a token.
    Background:
      When the Owner adds Bob to the whitelist
    
    Scenario Outline: If <minter> mints a token, the balance is updated
      When a <minter> mints a token to any user.
      Then the balance is <balance>.

    Examples:
      | minter | balance |
      | Owner  | 1       |
      | Alice  | 0       |
      | Bob    | 1       |
