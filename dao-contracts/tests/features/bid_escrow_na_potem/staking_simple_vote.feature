Feature: Reputation Token Staking in Simple Vote

    Background:
        Given deployed Reputation Token Contract
        And Account1 with 10 tokens
        And Account2 with 100 tokens
        And Account3 with 1000 tokens
        And Contract1 is whitelisted
        And Contract2 is whitelisted

    Scenario: Account wins informal voting
        When Contract1 stakes 5 in favour for Account1 for Vote1
        And Contract1 stakes 5 in favour for Account2 for Vote1
        And Contract1 stakes 5 against for Account3 for Vote1
        When Contract1 returns tokens for Vote1
        Then Account1 balance is 10 tokens
        And Account2 balance is 100 tokens
        And Account3 balance is 1000 tokens
        And total supply is 1110

    Scenario: Account wins formal voting
        When Contract1 stakes 5 in favour for Account1 for Vote1
        And Contract1 stakes 5 in favour for Account2 for Vote1
        And Contract1 stakes 5 against for Account3 for Vote1
        When Contract1 redistributes tokens for Vote1
        Then Account1 balance is 12 tokens
        And Account2 balance is 102 tokens
        And Account3 balance is 995 tokens
        And total supply is 1109

    Scenario: Only creator can redistribute or return tokens
        When Contract1 stakes 5 in favour for Account1 for Vote1
        Then Contract2 cant redistribute tokens for Vote1
        Then Contract2 cant return tokens for Vote1

    