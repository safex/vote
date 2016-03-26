//module that accepts a Poll (voting round)
//and accepts a list of votes


//count votes
//and validate poll with vote 


struct VotingOutcome {
	responses: Vec<String>,
	tally: Vec<i32>,
	outcome_hash: Vec<u8>,
	participating_addresses: Vec<String>,
	vote_weight: Vec<u32>,
}


impl VotingOutcome {
	///grab the contents of a poll from a file
	pub fn import_poll() {

	}

	///grab a directory containing votes and validate them against the poll
	pub fn import_votes() {
		//iterate through the directory for all .vote files, and parse out their contents also perform validation against the poll first step is the poll import_votes
		//then a prompt for the votes import, to be validated against the poll and against themselves
		//so will need to import votehash etc to validate all votes entirely.
	}
}