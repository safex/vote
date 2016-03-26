//! # Poll
//!
//! This application starts a new voting session
//!
extern crate safex;

use safex::genesis::key_generation::KeyPair;

fn main() {

	let keys = KeyPair::create().ok().expect("error");
	let pk = keys.public();
	let sk = keys.secret();
	println!("base64 private key: {:?}", KeyPair::private_key_tobase64(*sk));
	println!("base58 bitcoin address: {:?}", KeyPair::address_base58(pk));
}
