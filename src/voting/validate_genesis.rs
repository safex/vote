//module that accepts a Poll (voting round)
//and accepts a list of votes


//count votes
//and validate poll with vote 


use utils::dirs::{make_app_root_dir, touch};
use voting::poll_genesis::{PollRound, PollHash};
use utils::get_address_methods::{OmniList};

use safex::genesis::key_generation::KeyPair;
use std::env;


pub struct VotingOutcome {
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
	pub fn validate_outcome(pollround: PollRound) {
		//find the .poll file read and verify
		//find the .vote files read each and tally the vote
		let mut the_home_dir = String::new();
    	match env::home_dir() {
        	Some(ref p) => the_home_dir = p.display().to_string(),
        	None => println!("Impossible to get your home dir!")
    	}

    	let the_path1: String = the_home_dir.to_string() + "/validate_poll/";
    	let app_root: String = the_path1.to_string();
    	make_app_root_dir(app_root);


		
		//iterate through the directory for all .vote files, and parse out their contents also perform validation against the poll first step is the poll import_votes
		//then a prompt for the votes import, to be validated against the poll and against themselves
		//so will need to import votehash etc to validate all votes entirely.
	}

	pub fn poll_check(pollround: String) -> bool {
		let poll = PollRound::poll_fromjson(pollround);
		let poll_hash = poll.return_pollhash();
		let mut pollhash: Vec<u8> = Vec::new();
		for a in poll_hash.iter() {

			pollhash.push(*a);
		}

		let pollhash_clone = pollhash.clone();
		let pollhash_clone2 = pollhash.clone();

		let poll_choices = poll.return_pollchoices();
		let mut choice_vec: Vec<String> = Vec::new();
		for ch in poll_choices.iter() {
			choice_vec.push(ch.to_string());
		}

		let poll_terms = poll.return_theterms();
		let mut terms_str = String::from(poll_terms);


		let sp_nu = poll.return_spnum();

		let sig = poll.return_signature();
		let mut signa: Vec<u8> = Vec::new();
		for a in sig.iter() {

			signa.push(*a);
		}
		let the_sigclone = signa.clone();

		let origin_key = KeyPair::recover(signa, pollhash_clone);
		let hash160 = KeyPair::address_base58(&origin_key);

		let omnilist = poll.return_eligibleaddresses();
		let omnijson = omnilist.return_json();
		let omnilist = OmniList::omnilist_fromjson(omnijson);

		let the_pollhash_elems = PollHash {
			start_block: 0,
			end_block: 0,
			the_terms: terms_str,
			responses: choice_vec,
			sp_num: sp_nu,
			origin_pub: hash160,
			eligible_addresses: omnilist,
		};
		let duplicated_pollhash = the_pollhash_elems.return_hash();
		let clone_duphash = duplicated_pollhash.clone();
		if duplicated_pollhash.into_bytes() == pollhash_clone2 {
			println!("true");
		} else {
			println!("something is not right here");
			return false;
		}

		let sig_verification = KeyPair::verify(&origin_key, the_sigclone, clone_duphash.into_bytes());
		if sig_verification == true {
			println!("all good here");
			return true;
		} else {
			println!("signature error");
			return false;
		}
	}
}