Feature: VA Token
  Rule: A new instance has the default state
    Scenario: Deploy a new instance
      Then total supply of VA Token is 0 tokens
  
  Rule: A contract is set up
    Background:
      Given users
        | user    | whitelisted_in | is_va |
        | Alice   |                | false |
        | Bob     | VaToken        | false |
        | Holder  |                | true  |
        | Account |                | false |

    Scenario Outline: If <minter> mints a Va Token, the balance is updated
      When <minter> mints a VA Token to Account
      Then the Account's balance of VA Token is <balance>

    Examples:
      | minter | balance |
      | Alice  | 0       |
      | Bob    | 1       |
      | Owner  | 1       |

    Scenario: If Owner mints VA Token to Bob, he's balance is updated
      When Owner mints a VA Token to Bob
      Then the Bob's balance of VA Token is 1
      And VA Token with id 1 belongs to Bob
      And total supply of VA Token is 2 tokens
      
    Scenario: If a not whitelisted user VA Token to Bob, he's balance remains the same
      When Alice mints a VA Token to Bob
      Then the Bob's balance of VA Token is 0
      And total supply of VA Token is 1 token

    Scenario: If Owner mints the second VA Token to Holder, he's balance remains the same
      When Owner mints a VA Token to Holder
      Then the Holder's balance of VA Token is 1
      And total supply of VA Token is 1 token  
      
    Scenario: If a whitelisted user burns token, Holder's balance and total supply remain the same
      When Bob burns Holder's VA token
      Then the Holder's balance of VA Token is 0
      And total supply of VA Token is 0 tokens

    Scenario: If not whitelisted user burns token, Holder's balance and total supply remain the same
      When Alice burns Holder's VA token
      Then the Holder's balance of VA Token is 1
      And total supply of VA Token is 1 token