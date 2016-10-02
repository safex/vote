
use std::io::Read;

use rustc_serialize::{Decoder};
use rustc_serialize::json::{self};

use hyper::Client;
use hyper::header::Connection;


pub fn return_blockheight() -> u32 {
	get_blockheight()
}


fn get_blockheight() -> u32 {
    let client = Client::new();

    let url_string: String = "https://blockchain.info/q/getblockcount".to_string();

    let mut results = client.get(&url_string)
        .header(Connection::close())
        .send().expect("something went wrong");

    let mut payload = String::new();
    results.read_to_string(&mut payload).expect("could not read from response");

    //let height: u32 = payload.parse().ok().expect(error parsing);
    match payload.parse::<u32>() {
    	Ok(n) => n,
    	Err(..) => 0
    }
    
}

#[test]
fn work_it() {
	let the_height = return_blockheight();
} 