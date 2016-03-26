
use safex::genesis::key_generation::KeyPair;
use utils::get_address_methods::OmniList;
use utils::get_address_methods::get_omniwalletorg;

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
use std::cell::RefCell;

pub struct PollPersona {
	poller_keys: KeyPair,
	voting_round: PollRound,
}


impl PollPersona {
	pub fn import_keys() -> PollPersona {
		println!("input your private key");
		let mut input2 = String::new();
    	let stdin2 = io::stdin();
    	stdin2.lock().read_line(&mut input2).unwrap();

    	let trimmed = input2.trim_right_matches("\n");
    	let persona = PollPersona::persona_fromstring(trimmed.to_string());
    	persona
	}
	pub fn persona_fromstring(secret: String) -> PollPersona {
		let new_keys = KeyPair::keypair_frombase64(secret);
		let votings = PollRound::new();
		PollPersona {
			poller_keys: new_keys,
			voting_round: votings,
		}
	}
	pub fn return_keys(&self) -> &KeyPair {
		&self.poller_keys
	}
}

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct PollRound {
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
	//the sha256dhash of the poll in json format
	poll_hash: Vec<u8>,
	//public key of originator of the poll
	origin_pubkey: String,
	//signature of the voting round by originator
	origin_signature: Vec<u8>,
	//store a list of eligible addresses
	eligible_addresses: OmniList,
}

#[derive(Clone, RustcDecodable, RustcEncodable)]
pub struct PollHash {
	start_block: i32,
	end_block: i32,
	the_terms: String,
	responses: Vec<String>,
	sp_num: i32,
	origin_pub: String,
	eligible_addresses: OmniList,
}

impl PollHash {
	pub fn return_hash(&self) -> String {
    	let encoded = json::encode(&self).unwrap();
		let the_sha = Sha256dHash::from_data(&encoded.as_bytes());
		the_sha.to_string()
	}
}


impl PollRound {
	///forms a new PollRound object with 0 or empty parameters
	pub fn new() -> PollRound {
		PollRound {
			start_blockheight: 0,
			end_blockheight: 0,
			the_terms: String::new(),
			responses: Vec::new(),
			sp_number: 0,
			poll_hash: Vec::new(),
			origin_pubkey: String::new(),
			origin_signature: Vec::new(),
			eligible_addresses: OmniList::new(),
		}
	}


	///forms a new PollRound with the parameters specified
	pub fn new_wparams(the_terms: String, start_block: i32, end_block: i32, responses: Vec<String>, sp_num: i32, keys: &KeyPair, elig_address: OmniList) -> PollRound {
		
		let key_hash160 = KeyPair::address_base58(&keys.public);
		let elig_checkclone = elig_address.clone();
		let eligible_clone = elig_address.clone();
		if elig_checkclone.check_existence(key_hash160) == true {
			let key_hash160 = KeyPair::address_base58(&keys.public);
			let key_hash1602 = KeyPair::address_base58(&keys.public);
			let the_responseclone = responses.clone();
			let terms_clone = the_terms.clone();
			let the_pollhash_elems = PollHash {
				start_block: start_block,
				end_block: end_block,
				the_terms: the_terms,
				responses: responses,
				sp_num: sp_num,
				origin_pub: key_hash160,
				eligible_addresses: elig_address,
			};
			let the_pollhash = the_pollhash_elems.return_hash();
			let poll_hashclone = the_pollhash.clone();
			let poll_hash_sig = KeyPair::sign(&keys.secret, the_pollhash.into_bytes());
			let the_poll = PollRound {
				start_blockheight: start_block,
				//when the voting round ends
				end_blockheight: end_block,
				//the word describing the proposal
				the_terms: terms_clone,
				//the possible response strings
				responses: the_responseclone,
				//the number corresponding to the property address
				sp_number: sp_num,
				//the sha256dhash of the poll in json format
				poll_hash: poll_hashclone.into_bytes(),
				//public key of originator of the poll
				origin_pubkey: key_hash1602,
				//signature of the voting round by originator
				origin_signature: poll_hash_sig,
				//store a list of eligible addresses
				eligible_addresses: eligible_clone,
			};
			the_poll
		} else {
			print!("ineligible address attempting to make a poll, you need some coins first");
			PollRound {
				start_blockheight: 0,
				end_blockheight: 0,
				the_terms: String::new(),
				responses: Vec::new(),
				sp_number: 0,
				poll_hash: Vec::new(),
				origin_pubkey: String::new(),
				origin_signature: Vec::new(),
				eligible_addresses: OmniList::new(),
			}
		}
	}

	///the commandline function for forming a PollRound object with users' input writes to the home/make_polls/ directory
	pub fn make_poll() {
		let our_persona = PollPersona::import_keys();

		println!("please enter the SP number for the coin you will vote on. --  3 MaidSafeCoin; 56 SafeExchangeCoin;");
		let mut sp_in = String::new();
    	let stdin2 = io::stdin();
    	stdin2.lock().read_line(&mut sp_in).unwrap();
    	let sp_trim = sp_in.trim_right_matches("\n");
    	let sp_index: i32 = sp_trim.parse().ok().expect("invalid input");
		let omni_list = get_omniwalletorg(sp_index);
		let omni_listclone = omni_list.clone();

		let key_hash160 = KeyPair::address_base58(&our_persona.poller_keys.public);
		let key_hashclone = key_hash160.clone();
		let key_hashclone2 = key_hash160.clone();
		if omni_list.check_existence(key_hash160) == true {
			println!("you're going to form a poll");
			println!("please state the terms that you will suggest in your poll");
			let mut terms_in = String::new();
    		let stdin2 = io::stdin();
    		stdin2.lock().read_line(&mut terms_in).unwrap();
    		let terms_trim = terms_in.trim_right_matches("\n");

			let terms_inclone = terms_trim.clone();
			let terms_inclone2 = terms_trim.clone();

    		let mut select = 0;
    		let mut response_vec = Vec::new();
			println!("you're going to enter text based selections now these are the answers people choose as a vote");
    		while select == 0 {
    			println!("please enter a choice in the poll enter without quotes \"imdonenow\" when you no longer want to add anymore vote choices");
				let mut terms_in = String::new();
    			let stdin2 = io::stdin();
    			stdin2.lock().read_line(&mut terms_in).unwrap();

    			let terms_trim = terms_in.trim_right_matches("\n");

    			if terms_trim != "imdonenow".to_string() {
    				response_vec.push(terms_trim.to_string());
    			} else {
    				select += 1;
    			}
    		}
    		let mut response_clone = response_vec.clone();
    		println!("making poll hash");
    		let the_pollhash_elems = PollHash {
				start_block: 0,
				end_block: 0,
				the_terms: terms_inclone.to_string(),
				responses: response_vec,
				sp_num: sp_index,
				origin_pub: key_hashclone,
				eligible_addresses: omni_list,
			};
			let the_pollhash = the_pollhash_elems.return_hash();
    		println!("made poll hash");
			let poll_hashclone = the_pollhash.clone();
			let poll_hash_sig = KeyPair::sign(&our_persona.poller_keys.secret, the_pollhash.into_bytes());
    		println!("signed poll hash");
			let the_poll = PollRound {
				start_blockheight: 0,
				//when the voting round ends
				end_blockheight: 0,
				//the word describing the proposal
				the_terms: terms_inclone2.to_string(),
				//the possible response strings
				responses: response_clone,
				//the number corresponding to the property address
				sp_number: sp_index,
				//the sha256dhash of the poll in json format
				poll_hash: poll_hashclone.into_bytes(),
				//public key of originator of the poll
				origin_pubkey: key_hashclone2,
				//signature of the voting round by originator
				origin_signature: poll_hash_sig,
				//store a list of eligible addresses
				eligible_addresses: omni_listclone,
			};
			println!("{:?}", the_poll);
    		println!("about to write poll");
			the_poll.write_poll();
			println!("success");
		} else {
			println!("failure");;
		}
		//import private key,
		//answer questions
	}

	///returns a json encoded string from the PollRound struct
	pub fn return_jsonstring(&self) -> String {
    	let encoded = json::encode(&self).unwrap();
    	encoded
	}

	///returns a PollRound struct based on a json encoded string
	pub fn poll_fromjson(json: String) -> PollRound {
		let poll_data: PollRound = json::decode(&json).unwrap();
		poll_data
	}

	///returns the poll hash from the PollRound struct
	pub fn return_pollhash(&self) -> &[u8] {
		&self.poll_hash[..]
	}

	///returns the choices within the PollRound struct
	pub fn return_pollchoices(&self) -> &[String] {
		&self.responses
	}

	///returns the list of eligible addresses within the PollRound struct
	pub fn return_eligibleaddresses(&self) -> &OmniList {
		&self.eligible_addresses
	}

	///writes the poll to a file
	pub fn write_poll(&self) {
		let mut the_home_dir = String::new();
		let home_dirclone = the_home_dir.clone();
    	match env::home_dir() {
        	Some(ref p) => the_home_dir = p.display().to_string(),
        	None => println!("Impossible to get your home dir!")
    	}
    	let poll_hash = self.return_pollhash();
		let mut pollhash: Vec<u8> = Vec::new();
		for a in poll_hash.iter() {

			pollhash.push(*a);
		}
		let hash_path = String::from_utf8(pollhash).unwrap();

    	let path_string = String::from("/make_polls/");

    	let app_root = home_dirclone + "/make_polls/";
    	make_app_root_dir(&app_root);

    	let path_string2 = path_string + &hash_path;
    	let path_string3 = path_string2 + ".poll";
    	let path_string4 = the_home_dir + &path_string3;
    	let path = Path::new(&path_string4); 
    	println!("{:?}", path);;
		touch(&path).unwrap_or_else(|why| {
               println!("! {:?}", why.kind());
    	}); 

    	let display = "a";
		let mut file = match OpenOptions::new().read(true).write(true).open(path) {
            // The `description` method of `io::Error` returns a string that
            // describes the error
        	Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
       		Ok(file) => file,
    	};

    	let encoded = PollRound::return_jsonstring(self);
		let json_str = encoded.to_string();
		file.write_all(&encoded.as_bytes()).unwrap();
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

pub fn make_app_root_dir(rootname: &str) {
	let mut the_home_dir = String::new();

	match env::home_dir() {
   		Some(ref p) => the_home_dir = p.display().to_string(),
   		None => println!("Impossible to get your home dir!")
	}

	let the_other_part = rootname;
	let the_full_path = the_home_dir + the_other_part;
	match fs::create_dir(&the_full_path) {
		Err(why) => { 
			println!("{:?}", why.kind()); 
		},
		Ok(_) => { 	
			println!("making application directory"); 
		},
	}
}  

#[test]
fn test() {
	use utils::get_address_methods::get_omniwalletorg;
	let the_keys = KeyPair::create().unwrap();
	let omni_list = get_omniwalletorg(56);
	PollRound::new_wparams("hello".to_string(), 1, 2, vec!["hello".to_string(), "goodbye".to_string()], 3, &the_keys, omni_list);
}