
use utils::get_address_methods::OmniList;
use safex::genesis::key_generation::{KeyPair};

struct PollPersona {
	poller_keys: KeyPair,
	voting_round: VotingRound,
}


impl PollPersona {
	pub fn persona_fromstring(secret: String) -> PollPersona {
		let new_keys = KeyPair::keypair_frombase64(secret);
		let votings = VotingRound::new();
		PollPersona {
			poller_keys: new_keys,
			voting_round: votings,
		}
	}
}

#[derive(RustcDecodable, RustcEncodable)]
struct VotingRound {
	//when the voting round begins
	start_blockheight: i32,
	//when the voting round ends
	end_blockheight: i32,
	//the word describing the proposal
	the_terms: String,
	//the possible response strings
	responses: Vec<String>,
	//the number corresponding to the property address
	sp_number: i32,
	//the hash of the poll
	poll_hash: Vec<u8>,
	//public key of originator of the poll
	origin_pubkey: Vec<u8>,
	//signature of the voting round by originator
	origin_signature: Vec<u8>,
	//store a list of eligible addresses
	eligible_addresses: OmniList,
}


impl VotingRound {
	pub fn new() -> VotingRound {
		VotingRound {
			start_blockheight: 0,
			end_blockheight: 0,
			the_terms: String::new(),
			responses: Vec::new(),
			sp_number: 0,
			poll_hash: Vec::new(),
			origin_pubkey: Vec::new(),
			origin_signature: Vec::new(),
			eligible_addresses: OmniList::new(),
		}
	}



}




