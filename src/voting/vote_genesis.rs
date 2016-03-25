//this module spits out a structured vote as json data

//save the vote to a file


use safex::genesis::key_generation::KeyPair;
use utils::get_address_methods::OmniList;

use voting::poll_genesis::PollRound;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use bitcoin::util::hash::Sha256dHash;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Write;
use std::io;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::{BufRead};

pub struct VotePersona {
	voter_keys: KeyPair,
	voting_round: VoteRound,
}


impl VotePersona {
	pub fn import_keys() -> VotePersona {
		println!("input your private key");
		let mut input2 = String::new();
    	let stdin2 = io::stdin();
    	stdin2.lock().read_line(&mut input2).unwrap();

    	let trimmed = input2.trim_right_matches("\n");
    	let persona = VotePersona::persona_fromstring(trimmed.to_string());
    	persona
	}
	pub fn persona_fromstring(secret: String) -> VotePersona {
		let new_keys = KeyPair::keypair_frombase64(secret);
		let votings = VoteRound::new();
		VotePersona {
			voter_keys: new_keys,
			voting_round: votings,
		}
	}
	pub fn return_keys(&self) -> &KeyPair {
		&self.voter_keys
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

	
	pub fn form_vote() {
		let persona = import_keys();

		println!("please enter path of the poll you intend to vote on");
		let mut path = String::new();
    	let stdin = io::stdin();
    	stdin.lock().read_line(&mut path).unwrap();
    	let path_trim = path.trim_right_matches("\n");


    	let path = Path::new(&path_trim);
    	let display = "a";
   		let mut file = match OpenOptions::new().read(true).write(false).open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
        	Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        	Ok(file) => file,
    	};



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
		let mut the_home_dir = String::new();

    	match env::home_dir() {
        	Some(ref p) => the_home_dir = p.display().to_string(),
        	None => println!("Impossible to get your home dir!")
    	}
    	let vote_hash = self.return_votehash();
		let mut votehash: Vec<u8> = Vec::new();
		for a in vote_hash.iter() {

			votehash.push(*a);
		}
		let hash_path = String::from_utf8(votehash).unwrap();
    	let path_string = String::from("/make_votes/");
    	let path_string3 = the_home_dir + &path_string + &hash_path + &".vote".to_string();
    	let path = Path::new(&path_string3); 
		touch(&Path::new(path)).unwrap_or_else(|why| {
               println!("! {:?}", why.kind());
    	}); 

    	let display = "a";
		let mut file = match OpenOptions::new().read(true).write(true).open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
        	Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
       		Ok(file) => file,
    	};

    	let encoded = VoteRound::return_jsonstring(self);
		let json_str = encoded.to_string();
		file.write_all(&encoded.as_bytes()).unwrap();
	}

	pub fn return_votehash(&self) -> &[u8] {
		&self.vote_hash[..]
	}

	pub fn return_jsonstring(&self) -> String {
    	let encoded = json::encode(&self).unwrap();
    	encoded
	}



	
}

pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().write(true).read(true).create(true).open(path) {
        Ok(_) => { 
        	println!("making {:?}", path);
        	Ok(()) },
        Err(e) => Err(e),
    }
}

