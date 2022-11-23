Feature: KYC Token
  Background:
    Given users in KycToken contract
      | user   | is_whitelisted |
      | Alice  | false          |
      | Bob    | true           |
      | Holder | true           |

  Scenario: KYC Token initial state
    Then total supply is 0 tokens

  Rule: Only whitelisted user can mint a KYC Token  
    Scenario Outline: If <minter> mints a KYC Token, the balance is updated
      When <minter> mints a KYC Token to Holder
      Then the Holder's balance is <balance>

    Examples:
      | minter | balance |
      | Alice  | 0       |
      | Bob    | 1       |
      | Owner  | 1       |

  Rule: Owner can mint the first token to a user
    Scenario: If Owner mints KYC Token to Holder, he's balance is updated
      When Owner mints a KYC Token to Holder
      Then the Holder's balance is 1
      And Token with id 0 belongs to Holder
      And total supply is 1 token

  Rule: User can own only one KYC Token
    Background:
      Given Holder that owns a KYC Token
    
    Scenario: If Owner mints the second KYC Token for Holder, he's balance remains the same
      When Owner mints a KYC Token to Holder
      Then the Holder's balance is 1
  
  Rule: Only a whitelisted and approved user can burn a KYC Token
    Background:
      Given Holder that owns a KYC Token

    Scenario: If a user burns it's own token, his balance and total supply decrese
      When Holder burns Holder's token
      Then the Holder's balance is 0
      And total supply is 0 tokens
    
    Scenario: If whitelisted but not approved user burns token, Holder's balance and total supply remain the same
      When Bob burns Holder's token
      Then the Holder's balance is 1
      And total supply is 1 tokens

    Scenario: If not whitelisted user burns token, Holder's balance and total supply remain the same
      When Alice burns Holder's token
      Then the Holder's balance is 1
      And total supply is 1 token