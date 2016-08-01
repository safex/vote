extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate safex;
extern crate vote;
extern crate hyper;





use safex::genesis::key_generation::KeyPair;

use vote::voting::poll_genesis::{PollRound};
use vote::voting::vote_genesis::{VoteRound};
use vote::voting::validate_genesis::{VotingOutcome};
use vote::utils::get_address_methods::OmniList;
use vote::utils::get_address_methods::get_omniwalletorg;
use vote::utils::dirs::{make_app_root_dir, touch};

use iron::prelude::*;
use iron::{status, headers};
use iron::method::Method;
use iron::modifiers::Header;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::env;
use std::path::Path;
use std::fs::OpenOptions;
use std::error::Error;
use std::fs::File;
use std::io::Write;


#[derive(RustcEncodable, RustcDecodable)]
struct Respond {
	res: String,
}

fn main() {

	let uploaded_proposal = PollRound::new();
	let the_poll = Arc::new(Mutex::new(uploaded_proposal));
	let poll_clone = the_poll.clone();
	let poll_clone2 = the_poll.clone();

	let mut router = Router::new();


	router.post("/upload_proposal", move |r: &mut Request| receive_newproposal(r, &mut poll_clone.lock().unwrap()));

	///post a proposal to the server
	fn receive_newproposal(request: &mut Request, proposal: &mut PollRound) -> IronResult<Response> {
		//run validation against the proposal, if its valid, make a new directory with the name and hash
		//and write the proposal into the directory
		let mut payload = String::new();	

		let request_read = match request.body.read_to_string(&mut payload) {
			Ok(n) => "good".to_string(),
			Err(e) => "oops".to_string()
		};

		if request_read != "oops" {
			if VotingOutcome::poll_check(&payload) == true {

				let proposal = PollRound::poll_fromjson(payload);
				let poll_hash = proposal.return_pollhash();
				let mut pollhash: Vec<u8> = Vec::new();
				for a in poll_hash.iter() {

					pollhash.push(*a);
				}
				let hash_path = String::from_utf8(pollhash).unwrap();
				let proposal_name = proposal.return_thetitle();
				
				let mut the_home_dir = String::new();
    			match env::home_dir() {
        			Some(ref p) => the_home_dir = p.display().to_string(),
        			None => println!("Impossible to get your home dir!")
    			}
    			let name_hash = hash_path + &proposal_name;
				let home_dirclone = the_home_dir.clone();

    			let mut iter = name_hash.split_whitespace();
    			let mut name_hash = String::new();
    			for strings in iter {
    				name_hash.push_str(strings);
    			}

    			let proposal_root = "/".to_string() + &name_hash + "/";

    			let proposal_root_clone = proposal_root.clone();

    			make_app_root_dir(proposal_root.to_string());

    			println!("{:?}", &proposal_root_clone);

    			let proposal_write_path = home_dirclone + &proposal_root_clone +  &name_hash + ".poll";
    			let path = Path::new(&proposal_write_path); 
    			println!("{:?}", path);
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

    			let encoded = proposal.return_jsonstring();
				let json_str = encoded.to_string();
				let write_result = match file.write_all(&encoded.as_bytes()) {
					Ok(_) => "good",
					Err(_) => "not good"
				};

				if write_result == "good" {
						let resp = Respond { res: "Success Decoding and writing proposal to folder".to_string() };
						let resp_string = json::encode(&resp).unwrap();
						let mut response = Response::with((status::Ok, resp_string));
						response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
						println!("Sucess Decoding and writing proposal to folder");
						Ok(response)
				} else {
						let resp = Respond { res: "Error Writing proposal to server".to_string() };
						let resp_string = json::encode(&resp).unwrap();
						let mut response = Response::with((status::Ok, resp_string));
						response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
						println!("Error Writing proposal to server");
						Ok(response)
				}


			} else {
				let mut response = Response::with((status::Ok, "error with proposal file check"));
				response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
				println!("error with proposal file check");
				Ok(response)
			}
		} else {
			let mut response = Response::with((status::Ok, "error at read"));
			response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
			println!("error at read");
			Ok(response)
		}
		
	}



	//need a function to receive votes, and receive as part of the body the proposal to vote on name and hash
	//locate the directory with the proposal name and hash and then
	//validate the vote and then store the vote in that directory.



    Iron::new(router).http("localhost:3100").unwrap();
}
