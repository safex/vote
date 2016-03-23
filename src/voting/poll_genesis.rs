
use safex::genesis::key_generation::KeyPair;
use utils::get_address_methods::OmniList;


use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};

use bitcoin::util::hash::Sha256dHash;

struct PollPersona {
	poller_keys: KeyPair,
	voting_round: PollRound,
}


impl PollPersona {
	pub fn persona_fromstring(secret: String) -> PollPersona {
		let new_keys = KeyPair::keypair_frombase64(secret);
		let votings = PollRound::new();
		PollPersona {
			poller_keys: new_keys,
			voting_round: votings,
		}
	}
}

#[derive(RustcDecodable, RustcEncodable)]
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
	pub fn new_wparams(the_terms: String, start_block: i32, end_block: i32, responses: Vec<String>, sp_num: i32, keys: KeyPair, elig_address: OmniList) -> PollRound {
		
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

	pub fn return_jsonstring(&self) -> String {
    	let encoded = json::encode(&self).unwrap();
    	encoded
	}

	pub fn poll_fromjson(json: String) -> PollRound {
		let poll_data: PollRound = json::decode(&json).unwrap();
		poll_data
	}

	pub fn return_pollhash(&self) -> &[u8] {
		&self.poll_hash[..]
	}

	pub fn return_pollchoices(&self) -> &[String] {
		&self.responses
	}
	pub fn write_poll(&self) {

	}
}



#[test]
fn test() {
	use utils::get_address_methods::get_omniwalletorg;
	let the_keys = KeyPair::create().unwrap();
	let omni_list = get_omniwalletorg(56);
	PollRound::new_wparams("hello".to_string(), 1, 2, vec!["hello".to_string(), "goodbye".to_string()], 3, the_keys, omni_list);
}