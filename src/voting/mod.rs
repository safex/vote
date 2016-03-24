
use utils::get_address_methods::OmniList;

use bitcoin::util::hash::Sha256dHash;
use safex::genesis::key_generation::KeyPair;

use rustc_serialize::{Decodable, Decoder};
use rustc_serialize::json::{self, ToJson, Json};



pub mod vote_genesis;
pub mod validate_genesis;
pub mod poll_genesis;