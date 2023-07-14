mod variable_repository_contract {
    odra::casper::codegen::gen_contract!(
        dao::core_contracts::VariableRepositoryContract,
        "variable_repository_contract"
    );
}
mod admin_contract {
    odra::casper::codegen::gen_contract!(dao::voting_contracts::AdminContract, "admin_contract");
}
mod reputation_contract {
    odra::casper::codegen::gen_contract!(
        dao::core_contracts::ReputationContract,
        "reputation_contract"
    );
}
mod va_nft_contract {
    odra::casper::codegen::gen_contract!(dao::core_contracts::VaNftContract, "va_nft_contract");
}
mod kyc_nft_contract {
    odra::casper::codegen::gen_contract!(dao::core_contracts::KycNftContract, "kyc_nft_contract");
}
mod cspr_rate_provider_contract {
    odra::casper::codegen::gen_contract!(
        dao::utils_contracts::CSPRRateProviderContract,
        "cspr_rate_provider_contract"
    );
}
mod dao_ids_contract {
    odra::casper::codegen::gen_contract!(dao::utils_contracts::DaoIdsContract, "dao_ids_contract");
}
mod reputation_voter_contract {
    odra::casper::codegen::gen_contract!(
        dao::voting_contracts::ReputationVoterContract,
        "reputation_voter_contract"
    );
}
mod kyc_voter_contract {
    odra::casper::codegen::gen_contract!(
        dao::voting_contracts::KycVoterContract,
        "kyc_voter_contract"
    );
}
mod repo_voter_contract {
    odra::casper::codegen::gen_contract!(
        dao::voting_contracts::RepoVoterContract,
        "repo_voter_contract"
    );
}
mod simple_voter_contract {
    odra::casper::codegen::gen_contract!(
        dao::voting_contracts::SimpleVoterContract,
        "simple_voter_contract"
    );
}
mod slashing_voter_contract {
    odra::casper::codegen::gen_contract!(
        dao::voting_contracts::SlashingVoterContract,
        "slashing_voter_contract"
    );
}
mod bid_escrow_contract {
    odra::casper::codegen::gen_contract!(
        dao::bid_escrow::contract::BidEscrowContract,
        "bid_escrow_contract"
    );
}
mod onboarding_request_contract {
    odra::casper::codegen::gen_contract!(
        dao::voting_contracts::OnboardingRequestContract,
        "onboarding_request_contract"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    args.iter().skip(1).for_each(|arg| match arg.as_str() {
        "variable_repository_contract" => variable_repository_contract::main(),
        "admin_contract" => admin_contract::main(),
        "reputation_contract" => reputation_contract::main(),
        "va_nft_contract" => va_nft_contract::main(),
        "kyc_nft_contract" => kyc_nft_contract::main(),
        "cspr_rate_provider_contract" => cspr_rate_provider_contract::main(),
        "dao_ids_contract" => dao_ids_contract::main(),
        "reputation_voter_contract" => reputation_voter_contract::main(),
        "kyc_voter_contract" => kyc_voter_contract::main(),
        "repo_voter_contract" => repo_voter_contract::main(),
        "simple_voter_contract" => simple_voter_contract::main(),
        "slashing_voter_contract" => slashing_voter_contract::main(),
        "bid_escrow_contract" => bid_escrow_contract::main(),
        "onboarding_request_contract" => onboarding_request_contract::main(),
        _ => println!("Please provide a valid module name!"),
    });
}
