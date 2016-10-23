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
	println!("WIF private key: {:?}", KeyPair::private_key_base58compressed(*sk));
	let compressed = KeyPair::private_key_base58compressed(*sk);
	println!("{:?}", compressed.len());
	let uncompressed = KeyPair::private_key_base58(*sk);
	println!("{:?}", uncompressed.len());
	println!("base58 bitcoin address: {:?}", KeyPair::address_base58(pk));
}
