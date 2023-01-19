Feature: CSPR Rate Provider provides the Fiat:CSPR ratio

  Scenario: Only owner can alter the current CSPR ratio
    Given the price of USDT is 10 CSPR
    Then Owner is the owner of CSPRRateProvider contract
    When Alice sets the price of USDT to 20 CSPR
    Then the price of USDT is 10 CSPR
    When Owner sets the price of USDT to 30 CSPR
    Then the price of USDT is 30 CSPR
