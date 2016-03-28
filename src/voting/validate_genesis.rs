//module that accepts a Poll (voting round)
//and accepts a list of votes


//count votes
//and validate poll with vote 


use utils::dirs::{make_app_root_dir, touch, return_dirpaths};
use voting::poll_genesis::{PollRound, PollHash};
use voting::vote_genesis::{VoteRound, VoteHash};
use utils::get_address_methods::{OmniList};

use safex::genesis::key_generation::KeyPair;

use std::env;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::io;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::{BufRead};

pub struct VotingOutcome {
	responses: Vec<String>,
	tally: Vec<i32>,
	outcome_hash: Vec<u8>,
	participating_addresses: Vec<VotePiece>,
	vote_weight: Vec<i32>,
}

pub struct VotePiece {
	vote_hash: Vec<u8>,
	vote_count: i32,
	vote_address: String,
}

impl VotingOutcome {
	///grab a directory containing votes and validate them against the poll
	pub fn validate_outcome() -> bool {
		//find the .poll file read and verify
		//find the .vote files read each and tally the vote
		let mut the_home_dir = String::new();
		let home_dirclone = the_home_dir.clone();
    	match env::home_dir() {
        	Some(ref p) => the_home_dir = p.display().to_string(),
        	None => println!("Impossible to get your home dir!")
    	}
    	


    	let app_root = home_dirclone + "/make_valid/";
    	make_app_root_dir(app_root);

    	//read poll 

    	println!("please enter path to the directory where reside the one poll and all supposed votes");
		let mut path = String::new();
    	let stdin = io::stdin();
    	stdin.lock().read_line(&mut path).unwrap();
    	let path_trim = path.trim_right_matches("\n");

    	let path = Path::new(&path_trim);

    	let all_paths = return_dirpaths(&path);
    	let mut vote_paths = Vec::new();
    	let mut poll_path = String::new();
    	for paths in all_paths {

			let last_five: Vec<char> = paths.chars().collect();
			let len = last_five.len();
			let mut index = len - 5;
			let mut path_str = String::new();
			while index < len {
				path_str.push(last_five[index]);
				index += 1;
			}
			if path_str == ".poll".to_string() {
				poll_path = paths;
				println!("found the poll");
			} else if path_str == ".vote".to_string() {
				vote_paths.push(paths);
			}
    	} 
    	if poll_path.len() < 1 {
    		println!("error with poll_path none found");
    		false
    	} else {
    		//open up the poll
    		let the_path = Path::new(&poll_path);
    		let poll = PollRound::return_pollfromfile(the_path);
    		let poll_jsonstr = poll.return_jsonstring();
    		let poll_check = VotingOutcome::poll_check(poll_jsonstr);
    		if poll_check == true {

    			let responses = poll.return_pollchoices();
    			let poll_choices: Vec<String> = responses.to_vec();
    			let poll_choicesclone = poll_choices.clone();
    			let mut the_addresses: Vec<String> = Vec::new();

    			let mut the_pieces = Vec::new();

    			let mut the_tally = Vec::new();
				let mut the_tallybalance = Vec::new();

    			let omni_list = poll.return_eligibleaddresses();

    			for path in vote_paths {

    				let voter = VoteRound::return_votefromfile(Path::new(&path));
    				let vote_jsonstr = voter.return_jsonstring();
    				let vote_check = VotingOutcome::vote_check(vote_jsonstr);
    				if vote_check == true {
    					let vote_hash = voter.return_votehash();
    					let vote_pubclone = voter.vote_publickey.clone();
    					let vote_pubclone1 = voter.vote_publickey.clone();
    					let vote_count = omni_list.return_balance(vote_pubclone1);
    					the_tallybalance.push(vote_count);
    					the_tally.push(voter.vote_message.to_string());
    					let vote_address = vote_pubclone.to_string();
    					let the_piece = VotePiece {
							vote_hash: vote_hash.to_vec(),
							vote_count: vote_count,
							vote_address: vote_address,
						};
						the_pieces.push(the_piece);

    				} else {
    					println!("something was wrong with that vote");
    				}
    			}


    			let mut final_count: Vec<i32> = Vec::new();
    			let mut index = 0;
    			for choice in poll_choices {
    				final_count.push(0);
    				for a in 0..the_tally.len() {
    					let int = a;
    					if choice == the_tally[a] {
    						final_count[index] += the_tallybalance[int];
    					}
    				}
    				index += 1;
    			}
    			for a in 0..final_count.len() {
    				println!("{:?}", final_count[a]);
    				println!("{:?}", poll_choicesclone[a]);
    			}


    			true
    		}	else {
    			false
    		}
    	}

    	//read all votes
		
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

	pub fn vote_check(voteround: String) -> bool {
		let vote = VoteRound::vote_fromjson(voteround);
		let vote_hash = vote.return_votehash();
		let mut votehash: Vec<u8> = Vec::new();
		for a in vote_hash.iter() {

			votehash.push(*a);
		}

		let votehash_clone = votehash.clone();
		let votehash_clone2 = votehash.clone();
		let poll_hash = vote.return_pollhash();
		let mut pollhash: Vec<u8> = Vec::new();
		for a in poll_hash.iter() {

			pollhash.push(*a);
		}

		let pollhash_clone = pollhash.clone();
		let pollhash_clone2 = pollhash.clone();

		let vote_msg = vote.return_votemsg();
		let vote_index = vote.return_voteindex();

		let sig = vote.return_signature();
		let mut signa: Vec<u8> = Vec::new();
		for a in sig.iter() {

			signa.push(*a);
		}
		let the_sigclone = signa.clone();

		let origin_key = KeyPair::recover(signa, votehash_clone);
		let hash160 = KeyPair::address_base58(&origin_key);
		let vote_hash_elem = VoteHash {
			poll_hash: pollhash_clone,
			vote_message: vote_msg,
			vote_msgindex: vote_index,
			vote_publickey: hash160,
		};
		let vote_ahash = vote_hash_elem.return_hash();
		let vote_hashclone3 = vote_ahash.clone();
		let vote_hashbytes = vote_ahash.into_bytes();
		if vote_hashbytes == votehash_clone2 {
			println!("true");
		} else {
			println!("something is not right here hash error on a vote");
			return false;
		}
		let sig_verification = KeyPair::verify(&origin_key, the_sigclone, vote_hashclone3.into_bytes());
		if sig_verification == true {
			println!("all good here");
			return true;
		} else {
			println!("signature error on a vote");
			return false;
		}
	}
}