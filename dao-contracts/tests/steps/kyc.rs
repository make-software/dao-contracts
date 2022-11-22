use casper_types::U256;
use cucumber::{given, then, when, World};

use crate::common::DaoWorld;

#[then(expr = "total supply is {int} tokens")]
fn total_reputation(w: &mut DaoWorld, expected_total_supply: u32) {
    let total_supply = w.kyc_token.total_supply();
    assert_eq!(
        total_supply,
        U256::from(expected_total_supply)
    );
}

// use std::str::FromStr;

// use cucumber::Parameter;

// #[derive(Debug, Default)]
// struct Cat {
//     pub hungry: State,
// }

// impl Cat {
//     fn feed(&mut self) {
//         self.hungry = State::Satiated;
//     }
// }

// #[derive(Debug, Default, Parameter)]
// // NOTE: `name` is optional, by default the lowercased type name is implied.
// #[param(name = "hungriness", regex = "hungry|satiated")]
// enum State {
//     Hungry,
//     #[default]
//     Satiated,
// }

// // NOTE: `Parameter` requires `FromStr` being implemented.
// impl FromStr for State {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(match s {
//             "hungry" => Self::Hungry,
//             "satiated" => Self::Satiated,
//             invalid => return Err(format!("Invalid `State`: {invalid}")),
//         })
//     }
// }

// #[derive(Debug, Default, World)]
// pub struct AnimalWorld {
//     cat: Cat,
// }

// #[given(expr = "a {hungriness} cat")]
// fn hungry_cat(world: &mut AnimalWorld, state: State) {
//     world.cat.hungry = state;
// }

// #[when(expr = "I feed the cat {int} time(s)")]
// fn feed_cat(world: &mut AnimalWorld, times: u8) {
//     for _ in 0..times {
//         world.cat.feed();
//     }
// }

// #[then("the cat is not hungry")]
// fn cat_is_fed(world: &mut AnimalWorld) {
//     assert!(matches!(world.cat.hungry, State::Satiated));
// }
