//! # Poll
//!
//! This application starts a new voting session
//!
extern crate vote;

use vote::voting::poll_genesis::{PollRound};

fn main() {

	PollRound::make_poll();

}
