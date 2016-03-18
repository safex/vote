
use std::io;
use std::io::Read;

use rustc_serialize::{Decoder};
use rustc_serialize::json::{self};

use hyper::Client;
use hyper::header::Connection;


///Omniwallet.org api balances and relative addresses
#[derive(RustcDecodable, RustcEncodable)]
pub struct SingleData {
    pub balance: i32,
    pub reserved_balance: i32,
    pub address: String,
}

pub fn get_omniwalletorg(spindex: i32) -> Vec<SingleData> {
	let client = Client::new();

	let url_string: String = "https://www.omniwallet.org/v1/mastercoin_verify/addresses?currency_id=".to_string() + &spindex.to_string();

    let mut results = client.get(&url_string)
    	.header(Connection::close())
    	.send().expect("something went wrong");

    let mut payload = String::new();
    results.read_to_string(&mut payload).expect("could not read from response");

   	let mut int_new = 0;

    let decoded: Vec<SingleData> = json::decode(&payload).unwrap();
    decoded
}
