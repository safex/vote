extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate safex;
extern crate vote;
extern crate hyper;





use safex::genesis::key_generation::KeyPair;

use vote::voting::poll_genesis::{PollRound};
use vote::voting::vote_genesis::{VoteRound};
use vote::utils::get_address_methods::OmniList;
use vote::utils::get_address_methods::get_omniwalletorg;
use vote::utils::get_blockheight::return_blockheight;

use iron::prelude::*;
use iron::{status, headers};
use iron::method::Method;
use iron::modifiers::Header;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Arc, Mutex};

#[derive(RustcEncodable, RustcDecodable)]
struct Import {
	wif: String,
}

#[derive(RustcEncodable, RustcDecodable)]
struct ReturnPub {
	pub_key: String,
}

#[derive(RustcEncodable, RustcDecodable)]
struct PartialVote {
	vote_index: i32,
}

#[derive(RustcEncodable, RustcDecodable)]
struct PartialProposal {
	title: String,
	terms: String,
	choices: Vec<String>,
	end_date: u32,
}

fn main() {

	let place_key = KeyPair::create().unwrap();
	let the_key = Arc::new(Mutex::new(place_key));
	let key_clone = the_key.clone();
	let key_clone2 = the_key.clone();
	let key_clone3 = the_key.clone();

	let place_poll = PollRound::new();
	let the_poll = Arc::new(Mutex::new(place_poll));
	let poll_clone = the_poll.clone();
	let poll_clone2 = the_poll.clone();

	let place_vote = VoteRound::new();
	let the_vote = Arc::new(Mutex::new(place_vote));
	let vote_clone = the_vote.clone();

	let new_prop = PollRound::new();
	let the_prop = Arc::new(Mutex::new(new_prop));
	let prop_clone = the_prop.clone();
	let prop_clone2 = the_prop.clone();

	let mut router = Router::new();

	router.get("/getpub", move |r: &mut Request| get_pub(r, &the_key.lock().unwrap()));
	router.post("/setkey", move |r: &mut Request| set_key(r, &mut key_clone.lock().unwrap()));

	router.post("/setvote", move |r: &mut Request| set_vote(r, &key_clone2.lock().unwrap(), &the_poll.lock().unwrap(), &mut vote_clone.lock().unwrap()));
	router.get("/getvote", move |r: &mut Request| get_vote(r, &the_vote.lock().unwrap()));

	router.post("/setproposal", move |r: &mut Request| set_proposal(r, &mut poll_clone.lock().unwrap()));
	router.get("/getsetproposal", move |r: &mut Request| get_proposal(r, &poll_clone2.lock().unwrap()));

	router.post("/makeproposal", move |r: &mut Request| make_set_proposal(r, &mut prop_clone.lock().unwrap(), &key_clone3.lock().unwrap()));
	router.get("/getproposal", move |r: &mut Request| get_proposal(r, &prop_clone2.lock().unwrap()));


	///returns the base58 address corresponding to imported WIF 
	fn get_pub(_: &mut Request, key: &KeyPair) -> IronResult<Response> {
		let pub_key = KeyPair::address_base58(&key.public);
        let payload = json::encode(&pub_key).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    ///imports the private key
	fn set_key(request: &mut Request, key: &mut KeyPair) -> IronResult<Response> {
		let mut payload = String::new();
		request.body.read_to_string(&mut payload).unwrap();
		let import_hold: Import = json::decode(&payload).unwrap();
		*key = KeyPair::keypair_frombase58wif(import_hold.wif);

		let mut response = Response::with((status::Ok, "all good here"));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
		println!("hit the server");
		Ok(response)
	}

	///sets the proposal that will be voted on
	fn set_proposal(request: &mut Request, proposal: &mut PollRound) -> IronResult<Response> {
		let mut payload = String::new();
		request.body.read_to_string(&mut payload).unwrap();
		*proposal = json::decode(&payload).unwrap();

		let mut response = Response::with((status::Ok, "all good here"));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
		println!("hit the server");
		Ok(response)
	}

	///forms a vote based on the set proposal
	fn set_vote(request: &mut Request, key: &KeyPair, proposal: &PollRound, vote: &mut VoteRound) -> IronResult<Response> {
		let mut payload = String::new();
		request.body.read_to_string(&mut payload).unwrap();
		let vote_index: PartialVote = json::decode(&payload).unwrap();
		*vote = VoteRound::vote_newparam(proposal.return_jsonstring(), key, vote_index.vote_index);

		let mut response = Response::with((status::Ok, "all good here"));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
		println!("hit the server");
		Ok(response)
	}

	///reads back the set vote
	fn get_vote(_: &mut Request, vote: &VoteRound) -> IronResult<Response> {
		let payload = json::encode(&vote).unwrap();
        Ok(Response::with((status::Ok, payload)))
	}

	///makes a new proposal and sets it
	fn make_set_proposal(request: &mut Request, prop: &mut PollRound, keys: &KeyPair) -> IronResult<Response> {
		let mut payload = String::new();
		request.body.read_to_string(&mut payload).unwrap();
		let partial_prop: PartialProposal = json::decode(&payload).unwrap();

		let omnis = get_omniwalletorg(56);
		*prop = PollRound::new_wparams(partial_prop.title, partial_prop.terms, return_blockheight(), partial_prop.end_date, partial_prop.choices, 56, keys, omnis);

		let mut response = Response::with((status::Ok, "all good here"));
		response.set_mut(Header(headers::AccessControlAllowOrigin::Any));
		println!("hit the server");
		Ok(response)
	}

	///return new made proposal
	fn get_proposal(_: &mut Request, prop: &PollRound) -> IronResult<Response> {
		let payload = json::encode(&prop).unwrap();
		Ok(Response::with((status::Ok, payload)))
	}


	//store the key here in a variable

	//post import key and store ^^

	//collect content, validate and produce the proposal file

	//finally make a vote - open directory maker to select where to save the vote file.


    Iron::new(router).http("localhost:3000").unwrap();
}
