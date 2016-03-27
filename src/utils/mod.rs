use std::io;
use std::io::Read;

use rustc_serialize::{Decoder};
use rustc_serialize::json::{self};

use hyper::Client;
use hyper::header::Connection;

pub mod get_address_methods;
pub mod dirs;