use cucumber::{given, when, gherkin::Step};
use casper_dao_contracts::voting::Choice;

use crate::common::DaoWorld;
use crate::common::helpers::{match_choice, multiplier, to_rep, value_to_bytes};

