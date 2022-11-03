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

