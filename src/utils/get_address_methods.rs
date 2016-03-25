
use std::io;
use std::io::Read;

use rustc_serialize::{Decoder};
use rustc_serialize::json::{self};

use hyper::Client;
use hyper::header::Connection;


///Omniwallet.org api balances and relative addresses
#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct SingleData {
    pub balance: i32,
    pub reserved_balance: i32,
    pub address: String,
}

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct OmniList {
    address_list: Vec<SingleData>
}


impl OmniList {

    pub fn new() -> OmniList {
        OmniList {
            address_list: Vec::new(),
        }
    }

    ///returns true or false if a given address exists in the list
    pub fn check_existence(&self, address: String) -> bool {
        for addresses in &self.address_list {
            if addresses.address == address {
                return true;
            }
        }
        false
    }

    ///return list of addresses and balances from OmniList object
    pub fn return_list(&self) -> &Vec<SingleData> {
        &self.address_list
    }


}

pub fn get_omniwalletorg(spindex: i32) -> OmniList {
    let client = Client::new();

    let url_string: String = "https://www.omniwallet.org/v1/mastercoin_verify/addresses?currency_id=".to_string() + &spindex.to_string();

    let mut results = client.get(&url_string)
        .header(Connection::close())
        .send().expect("something went wrong");

    let mut payload = String::new();
    results.read_to_string(&mut payload).expect("could not read from response");

    let mut int_new = 0;

    let decoded: Vec<SingleData> = json::decode(&payload).unwrap();
    let mut the_list = OmniList::new();
    the_list.address_list = decoded;
    the_list
}

#[test]
fn test() {

    let the_list = get_omniwalletorg(56);
    let contains_or = the_list.check_existence("15N8mbsRwiwyQpsTUcGfETpStYkTFjcHvh".to_string());
    assert_eq!(true, contains_or);

}