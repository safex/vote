//! # Validate
//!
//! This application returns the results of a Poll and validates all participating transactions
//!
extern crate vote;

use vote::voting::validate_genesis::{VotingOutcome};

fn main() {
	
    println!("{:?}", VotingOutcome::validate_outcome());
}
