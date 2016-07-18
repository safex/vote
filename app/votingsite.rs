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

use iron::prelude::*;
use iron::{status, headers};
use iron::method::Method;
use iron::modifiers::Header;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};



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

		let this: String = match request.body.read_to_string(&mut payload) {
			Ok(n) => payload.to_string(),
			Err(e) => "oops".to_string()
		};
		if this != "oops" {
			let json_proposal = match json::decode(&payload) {
				Ok(n) => n,
				Err(e) => "oops".to_string()
			};
			if json_proposal != "oops" {
				let our_proposal = PollRound::poll_fromjson(json_proposal);
				if VotingOutcome::poll_check(our_proposal.return_jsonstring()) == true {
					//make a new directory with the name of the proposal + its hash
					//get the has and make a directory and a name for this proposal



					let mut response = Response::with((status::Ok, "all good here"));
					response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
					println!("wrote proposal");
					Ok(response)
				} else {
				let mut response = Response::with((status::Ok, "error at poll_check"));
				response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
				println!("error at poll_check");
				Ok(response)
			}
			} else {
				let mut response = Response::with((status::Ok, "error at json_proposal"));
				response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
				println!("error at json_proposal");
				Ok(response)
			} 
		} else {
				let mut response = Response::with((status::Ok, "error at read"));
				response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
				println!("error at read");
				Ok(response)
		}
		//*proposal = try!(json::decode(&payload).unwrap());
	}



	//need a function to receive votes, and receive as part of the body the proposal to vote on name and hash
	//locate the directory with the proposal name and hash and then
	//validate the vote and then store the vote in that directory.


	
    Iron::new(router).http("localhost:3100").unwrap();
}
