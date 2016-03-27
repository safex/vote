use std::error::Error;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::env;
use std::io::Write;
use std::io;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::{BufRead};


pub fn touch(path: &Path) -> io::Result<()> {
    match OpenOptions::new().write(true).read(true).create(true).open(path) {
        Ok(_) => { 
        	println!("making {:?}", path);
        	Ok(()) },
        Err(e) => Err(e),
    }
}

pub fn return_dirpaths(path: &Path) -> Vec<String> {
	let paths = fs::read_dir(path).unwrap();
	let mut the_paths = Vec::new();
   	for path in paths {
        //println!("Name: {}", path.unwrap().path().display());
        the_paths.push(path.unwrap().path().display().to_string());
    }
    the_paths
}


pub fn make_app_root_dir(rootname: String) {
	let mut the_home_dir = String::new();

	match env::home_dir() {
   		Some(ref p) => the_home_dir = p.display().to_string(),
   		None => println!("Impossible to get your home dir!")
	}

	let the_other_part = rootname;
	let the_full_path = the_home_dir + &the_other_part;
	match fs::create_dir(&the_full_path) {
		Err(why) => { 
			println!("{:?}", why.kind()); 
		},
		Ok(_) => { 	
			println!("making application directory"); 
		},
	}
}  