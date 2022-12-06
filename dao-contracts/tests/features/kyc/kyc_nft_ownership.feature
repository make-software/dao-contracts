Feature: KYC Token ownership management
  Background:
    Given users
      | user    | whitelisted_in |
      | Alice   |                |
      | Bob     | KycToken       |

  Scenario: Deploy a new instance
    Then Deployer is the owner of KycToken contract
    And Deployer is whitelisted in KycToken contract
  
  Rule: Only the current owner can change ownership
    Scenario: The current owner changes ownership
      When Deployer sets Alice as a new owner of KycToken contract
      Then Alice is the owner of KycToken contract
      And Alice is whitelisted in KycToken contract
      And Deployer is whitelisted in KycToken contract

    Scenario: A user out of the whitelist claims ownership
      When Alice sets Alice as a new owner of KycToken contract
      Then Deployer is the owner of KycToken contract
    
    Scenario: A whitelisted user claims ownership
      When Bob sets Bob as a new owner of KycToken contract
      Then Deployer is the owner of KycToken contract
  
  Rule: Only the current owner can remove a user from the whitelist
    Scenario: Owners removes from the whitelist
      When Deployer removes Bob from whitelist in KycToken contract
      Then Bob is not whitelisted in KycToken contract

    Scenario: Non-owner removes from the whitelist
      When Bob removes Bob from whitelist in KycToken contract
      Then Bob is whitelisted in KycToken contract

  Rule: Only the current owner can remove a user from the whitelist
    Scenario: Owner adds to the whitelist
      When Deployer adds Alice to whitelist in KycToken contract
      Then Alice is whitelisted in KycToken contract

    Scenario: A user adds to the whitelist
      When Bob adds Alice to whitelist in KycToken contract
      Then Alice is not whitelisted in KycToken contract

  