//this module spits out a structured vote as json data

//save the vote to a file


use safex::genesis::key_generation::KeyPair;
use utils::get_address_methods::OmniList;


use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use bitcoin::util::hash::Sha256dHash;

struct VotePersona {
	voter_keys: KeyPair,
	voting_round: VoteRound,
}


impl VotePersona {
	pub fn persona_fromstring(secret: String) -> VotePersona {
		let new_keys = KeyPair::keypair_frombase64(secret);
		let votings = VoteRound::new();
		VotePersona {
			voter_keys: new_keys,
			voting_round: votings,
		}
	}
}

#[derive(RustcDecodable, RustcEncodable)]
struct VoteRound {
	poll_hash: Vec<u8>,
	vote_hash: Vec<u8>,
	vote_message: String,
	vote_signature: Vec<u8>,
	vote_publickey: String,
}


impl VoteRound {
	pub fn new() -> VoteRound {
		VoteRound {
			poll_hash: Vec::new(),
			vote_hash: Vec::new(),
			vote_message: String::new(),
			vote_signature: Vec::new(),
			vote_publickey: String::new(),
		}
	}
	pub fn from_poll(poll_round: String) {
		
	}

//	Sha256dHash::from_data(&message[..]);


}