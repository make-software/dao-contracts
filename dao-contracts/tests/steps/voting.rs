use casper_dao_contracts::voting::Choice;
use cucumber::{gherkin::Step, given, when};

use crate::common::{
    helpers::{match_choice, multiplier, to_rep, value_to_bytes},
    DaoWorld,
};
