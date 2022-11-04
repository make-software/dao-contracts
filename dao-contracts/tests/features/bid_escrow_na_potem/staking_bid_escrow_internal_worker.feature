Feature: Reputation Token Staking for Bid Escrow Internal Worker

    Background:
        Given deployed Reputation Token Contract
        And Account1 with 10 tokens
        And Account2 with 100 tokens
        And Account3 with 1000 tokens
        And Contract1 is whitelisted
        And Contract2 is whitelisted

    Scenario: Reputation minting for succesfull voting
        When Contract1 sets 10 reputation reward for Vote1
        And Contract1 stakes 5 in favour for Account1 for Vote1
        And Contract1 stakes 5 in favour for Account2 for Vote1
        And Contract1 stakes 5 against for Account3 for Vote1
        And total supply is 1110
        When Contract1 redistributes tokens for Vote1 with policing rate 0.3
        Then Account1 balance is 21 tokens
        And Account2 balance is 103 tokens
        And Account3 balance is 995 tokens
        And total supply is 1119

    Scenario: Reputation minting for unsuccesfull voting
        When Contract1 sets 20 reputation reward for Vote1
        And Contract1 stakes 10 in favour for Account1 for Vote1
        And Contract1 stakes 50 in favour for Account2 for Vote1
        And Contract1 stakes 500 against for Account3 for Vote1
        And total supply is 1110
        When Contract1 redistributes tokens for Vote1 with policing rate 0.3
        Then Account1 balance is 0 tokens
        And Account2 balance is 50 tokens
        And Account3 balance is 1060 tokens
        And total supply is 1110

    Scenario: Reputation minting for no-quorum voting
        When Contract1 sets 20 reputation reward for Vote1
        And Contract1 stakes 10 in favour for Account1 for Vote1
        And Contract1 stakes 50 in favour for Account2 for Vote1
        And Contract1 stakes 500 against for Account3 for Vote1
        And total supply is 1110
        When Contract1 returns tokens for Vote1
        Then Account1 balance is 10 tokens
        And Account2 balance is 50 tokens
        And Account3 balance is 1000 tokens
        And total supply is 1110
