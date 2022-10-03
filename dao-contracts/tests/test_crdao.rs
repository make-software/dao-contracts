#[test]
fn test_crdao_deployment() {
    assert_eq!(true, false);
    // VA: 20 (configurable, const)
    
    // 1. Deploy Reputation Token Contract.
    // 1.1 Mint reputation token for initial list of users.

    // 2. Deploy Variable Repository Contract.
    // 2.1 Change both quorum to 30%
    // 2.2 Change voting durations to 1d.

    // 3. Deploy VA Token Contract.
    // 3.1 Mint VA tokens for every VA.

    // 4. Deploy KYC Token Contract.
    
    // 5. Deploy Reputation Voter.
    // 5.1 Whilelist in Reputation Token Contract.

    // 6. Deploy Repo Voter Contract.
    // 6.1 Whilelist in Reputation Token Contract.
    // 6.2 Whilelist in Variable Repository Contract.
    
    // 7. Deploy Onboarding Voter Contract.
    // 7.1 Whilelist in Reputation Token Contract.
    // 7.2 Whilelist in VA Token Contract

    // 8. Deploy KYC Voter Contract.
    // 8.1 Whilelist in Reputation Token Contract.
    // 8.2 Whilelist in KYC Token Contract

    // 9. Deploy Simple Voter Contract
    // 9.1 Whitelist in Reputation Token Contract.

    // 10. Deploy Admin Contract.
    // 10.1.1 Remove itself from the whitelist in Reputation Token Contract.
    // 10.1.2 Change Reputation Token Contract owner to Admin Contract.
    // 10.2.1 Remove itself from the whitelist in Variable Repository Contract.
    // 10.2.2 Change Variable Repository Contract owner to Admin Contract.
    // 10.3.1 Remove itself from the whitelist in VA Token Contract.
    // 10.3.2 Change VA Token Contract owner to Admin Contract.
    // 10.4.1 Remove itself from the whitelist in KYC Token Contract.
    // 10.4.2 Change KYC Token Contract owner to Admin Contract.

    // 11. Onboard new VA.
    // 11.1. Call create_voting on KYC Voter Contract as a VA to mint new KYC Token to a new user.
    // 11.2. All votes in faviour.
    // 11.3. Call create_voting on Onbaording Voter Contract as a VA to mint new VA Token to the same user.
    // 11.4. All votes in faviour.
    // 11.5. Call create_voting on Reputation Voter Contract as a VA to mint reputation tokens to the same user.
    // 11.6. All votes in faviour.

    // 12. The New VA 
    // 12.1 Call create_voting on Admin Contract to whilelist itself in Reputation Token.
    // 12.2 All votes against.
}