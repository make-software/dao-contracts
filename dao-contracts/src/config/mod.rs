//! System configuration.
//!
//! A configuration is a mix of [`Governance Variables`] and voting configuration.
//! DAO supports a few types of voting. Each type may have a slightly different configuration.
//! Once voting is created, until the end, voting relies on the system's state at the moment of voting creation.
//! It mitigates unexpected behavior during voting if the internal DAO state changes.
//!
//! [`Governance Variables`]: crate::variable_repository
mod builder;
mod configuration;
mod dao_configuration;
mod voting_configuration;

pub use builder::ConfigurationBuilder;
pub use configuration::Configuration;
use dao_configuration::DaoConfiguration;
use voting_configuration::VotingConfiguration;
