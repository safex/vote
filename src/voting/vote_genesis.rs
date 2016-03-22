//this module spits out a structured vote as json data

//save the vote to a file

struct VotePersona {
	poller_keys: KeyPair,
	voting_round: VoteRound,
}


impl VotePersona {
	pub fn persona_fromstring(secret: String) -> VotePersona {
		let new_keys = KeyPair::keypair_frombase64(secret);
		let votings = VoteRound::new();
		VotePersona {
			poller_keys: new_keys,
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

//	Sha256dHash::from_data(&message[..]);


}