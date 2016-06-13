extern crate iron;
extern crate router;
extern crate rustc_serialize;
extern crate safex;


use safex::genesis::key_generation::KeyPair;

use iron::prelude::*;
use iron::status;
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

fn main() {

	let place_key = KeyPair::create().unwrap();
	let the_key = Arc::new(Mutex::new(place_key));
	let key_clone = the_key.clone();

	let mut router = Router::new();

	router.get("/", move |r: &mut Request| get_pub(r, &the_key.lock().unwrap()));
	router.post("/importkey", move |r: &mut Request| set_key(r, &mut key_clone.lock().unwrap()));

	fn get_pub(_: &mut Request, key: &KeyPair) -> IronResult<Response> {
		let pub_key = KeyPair::address_base58(&key.public);
        let payload = json::encode(&pub_key).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

	fn set_key(request: &mut Request, key: &mut KeyPair) -> IronResult<Response> {
		let mut payload = String::new();
		request.body.read_to_string(&mut payload).unwrap();
		let import_hold: Import = json::decode(&payload).unwrap();
		*key = KeyPair::keypair_frombase58wif(import_hold.wif);
		Ok(Response::with(status::Ok))
	}

	//store the key here in a variable

	//post import key and store ^^

	//collect content, validate and produce the proposal file

	//finally make a vote - open directory maker to select where to save the vote file.


    Iron::new(router).http("localhost:3000").unwrap();
}


//need app 