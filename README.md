# contracts

Voting contract
    data:
    - voting_address - Address

Reputation contract
    data:
    - reputation_address - Address
    - owner - Address
    - whitelist - Vec<Address>

    methods:
        only_onwer:
        - change_onwership(new_owner: Address) 
        - add_to_whitelist(addr: Address)
        - remove_from_whitelist(addr: Address)
          
        whitelist_only:
        - mint
        - burn
        - transfer_from
        - stake

Variable Repository
    data:
        ...
    
    methods:
        only_onwer:
        - change_onwership(new_owner: Address) 
        - add_to_whitelist(addr: Address)
        - remove_from_whitelist(addr: Address)
        
        whitelist_only:
        - set_string(name: String, value: String)
        - set_u256(name: String, value: U256)

        all:
        - get_string(name: String) -> String
        - get_u256(name: String) -> U256

Master Voting Contract:
    - vote for whitelist changes.