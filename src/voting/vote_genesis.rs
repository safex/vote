//this module spits out a structured vote as json data

//save the vote to a file




#[derive(RustcDecodable, RustcEncodable)]
struct FormVote {
	poll_hash: Vec<u8>,
	vote_hash: Vec<u8>,
	vote_message: String,
	vote_signature: Vec<u8>,
	vote_publickey: String,
}


impl FormVote {

//	Sha256dHash::from_data(&message[..]);


}