//this module spits out a structured vote as json data

//save the vote to a file


use safex::genesis::key_generation::KeyPair;
use utils::get_address_methods::OmniList;

use voting::poll_genesis::PollRound;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use bitcoin::util::hash::Sha256dHash;

use std::io;
use std::io::Read;
use std::io::{BufRead};

pub struct VotePersona {
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

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct VoteHash {
	poll_hash: Vec<u8>,
	vote_message: String,
	vote_msgindex: i32,
	vote_publickey: String,
}

impl VoteHash {
	pub fn return_hash(&self) -> String {
    	let encoded = json::encode(&self).unwrap();
		let the_sha = Sha256dHash::from_data(&encoded.as_bytes());
		the_sha.to_string()
	}
}


#[derive(RustcDecodable, RustcEncodable)]
pub struct VoteRound {
	poll_hash: Vec<u8>,
	vote_hash: Vec<u8>,
	vote_message: String,
	vote_msgindex: i32,
	vote_signature: Vec<u8>,
	vote_publickey: String,
}


impl VoteRound {
	pub fn new() -> VoteRound {
		VoteRound {
			poll_hash: Vec::new(),
			vote_hash: Vec::new(),
			vote_message: String::new(),
			vote_msgindex: 0,
			vote_signature: Vec::new(),
			vote_publickey: String::new(),
		}
	}
	pub fn from_poll(&self, poll_round: String, persona: VotePersona) -> VoteRound {
		//get the poll's hash
		//need to validate the poll contents as well
		
		let poll = PollRound::poll_fromjson(poll_round);
		let poll_hash = poll.return_pollhash();
		let mut pollhash: Vec<u8> = Vec::new();
		for a in poll_hash.iter() {

			pollhash.push(*a);
		}

		let pollhash_clone = pollhash.clone();

		let poll_choices = poll.return_pollchoices();

		let vote_index = VoteRound::select_answer(poll_choices);
		let vote_string = poll_choices[vote_index as usize].to_string();
		let vstring_clone = vote_string.clone();

		let keys = persona.voter_keys;

		let pk_string = KeyPair::address_base58(&keys.public);
		let pkstr_clone = pk_string.clone();

		let vote_hash = VoteHash {
			poll_hash: pollhash,
			vote_message: vote_string,
			vote_msgindex: vote_index,
			vote_publickey: pk_string,
		};
		let vote_hash = vote_hash.return_hash();
		let vhash_clone = vote_hash.clone();

		let vote_signature = KeyPair::sign(&keys.secret, vote_hash.into_bytes());

		let the_vote = VoteRound {
			poll_hash: pollhash_clone,
			vote_hash: vhash_clone.into_bytes(),
			vote_message: vstring_clone,
			vote_msgindex: vote_index,
			vote_signature: vote_signature,
			vote_publickey: pkstr_clone,
		};
		the_vote

		//let poll_data: PollRound = json::decode(&poll_round).unwrap();
		//let poll_hash = 

	}
	pub fn select_answer(poll_choices: &[String]) -> i32 {
		print!("choices are: ");
		let mut index = 0;
		for choice in poll_choices.iter() {
			print!("index {:?}", choice);
			index += 1;
		}
		print!("enter the index number of your selection");
		let mut input2 = String::new();
    	let stdin2 = io::stdin();
    	stdin2.lock().read_line(&mut input2).unwrap();

    	let trimmed = input2.trim_right_matches("\n");
    	let the_index: i32 = trimmed.parse().ok().expect("invalid input");
    	the_index
	}

	pub fn write_vote(&self) {

	}
//	Sha256dHash::from_data(&message[..]);


}