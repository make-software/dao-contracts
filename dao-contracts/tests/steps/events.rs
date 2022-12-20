use casper_dao_utils::TestContract;
use cucumber::{gherkin::Step, then};

use crate::common::{
    params::{events::Event, Contract},
    DaoWorld,
};

#[then(expr = "{contract} contract emits events")]
fn assert_event(world: &mut DaoWorld, step: &Step, contract: Contract) {
    let table = step.table.as_ref().unwrap().rows.iter().skip(1);

    let total_events = match contract {
        Contract::KycToken => world.kyc_token.events_count(),
        Contract::KycVoter => world.kyc_voter.events_count(),
        Contract::VaToken => world.va_token.events_count(),
        Contract::ReputationToken => world.reputation_token.events_count(),
        Contract::BidEscrow => world.bid_escrow.events_count(),
        Contract::VariableRepository => world.variable_repository.events_count(),
        Contract::SlashingVoter => world.slashing_voter.events_count(),
        Contract::Admin => world.admin.events_count(),
        Contract::RepoVoter => world.repo_voter.events_count(),
        Contract::SimpleVoter => world.simple_voter.events_count(),
        Contract::ReputationVoter => world.reputation_voter.events_count(),
        Contract::Onboarding => world.onboarding.events_count(),
    };
    let expected_events_count = table.len() as i32;
    // In a scenario are given last n events, so we need to skip first #events-n events.
    let skipped_events = total_events - expected_events_count;

    for (idx, row) in table.enumerate() {
        let event: Event = row.into();
        let event_idx = idx as i32 + skipped_events;
        world.assert_event(&contract, event_idx, event);
    }
}
