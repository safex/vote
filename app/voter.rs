//! # Vote
//!
//! This application takes an imported private key, and allows someone to vote in a Poll
//!
extern crate vote;

use vote::voting::vote_genesis::{VoteRound};

fn main() {
	
    VoteRound::form_vote();

}
