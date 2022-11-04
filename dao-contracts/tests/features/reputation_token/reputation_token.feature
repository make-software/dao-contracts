Feature: Reputation Token
    Background:
        Given deployed Reputation Token Contract
    
    Scenario: Initial state of the token
        Then total supply is 0
        And Owner is set as an owner
        And Owner is whitelisted

    Scenario: Tokens minting
        When Owner mints 10 tokens for Account1
        Then Account1 balance is 10 tokens
        And Account1 has 0 tokens staked
        And total supply is 10

    Scenario: Tokens burning
        Given Account1 with 10 tokens
        When Owner burns 6 tokens from Account1
        Then Account1 balance is 4 tokens
        And total supply is 4

    Scenario: Tokens burning with debt
        Given Account1 with 10 tokens
        And Account1 has 8 tokens staked
        When Owner burns 5 tokens from Account1
        Then Account1 has 8 tokens staked
        And Account1 balance is 8
        And total supply is 8
        When Owner unstakes 8 tokens from Account1
        Then Account1 balance is 5
        And total supply is 5

    Scenario: Tokens slashed simple
        Given Account1 with 100 tokens
        And Account1 has 20 tokens staked for Vote1
        
        When Owner slashes 90 percent of Account1 tokens
        Then Account1 balance is 10 tokens
        And Account1 debt is 10
        And Account1 staked tokens is 20
        And total supply is 10
        
        When Onwner unstakes 20 tokens for Account1 Vote1
        Then Account1 balance is 10 tokens
        And Account1 debt is 0
        And Account1 staked tokens is 0
        And total supply is 10

    Scenario: Tokens slashed complex
        Given Account1 with 100 tokens
        And Account1 has 20 tokens staked for Vote1
        
        When Owner slashes 90 percent of Account1 tokens
        Then Account1 balance is 10 tokens
        And Account1 debt is 10
        And Account1 staked tokens is 20
        And total supply is 10

        When Owner rewards Account1 with 50 tokens
        Then Account1 balance is 60 tokens
        And Account1 debt is 0
        And Account1 staked tokens is 20
        And total supply is 60

        When Onwner unstakes 20 tokens for Account1 Vote1
        Then Account1 balance is 60 tokens
        And Account1 debt is 0
        And Account1 staked tokens is 0
        And total supply is 60

    Scenario: Tokens slashed 100 percent
        Given Account1 with 100 tokens
        And Account1 has 20 tokens staked for Vote1
        
        When Owner slashes 100 percent of Account1 tokens
        Then Account1 balance is 0 tokens
        And Account1 debt is 0
        And Account1 staked tokens is 20
        And total supply is 0

        When Onwner unstakes 20 tokens for Account1 Vote1
        Then Account1 balance is 0 tokens
        And Account1 debt is 0
        And Account1 staked tokens is 0
        And total supply is 0

    Scenario: Tokens slashed 100 percent complex
        Given BidEscrow is whitelisted
        And ReputationSlashed is whitelisted
        And balances are
        | account | REP balance | REP stake |
        | Account1 | 100        | 20        |

        And Account1 with 100 tokens
        And Account1 has 20 tokens staked for Vote1
        
        When Owner slashes 100 percent of Account1 tokens
        Then Account1 balance is 0 tokens
        And Account1 debt is 0
        And Account1 staked tokens is 0
        And total supply is 0

        When Owner redistributes tokens for Vote2 where  

        When Onwner unstakes 20 tokens for Account1 Vote1
        Then Account1 balance is 0 tokens
        And Account1 debt is 0
        And Account1 staked tokens is 0
        And total supply is 0