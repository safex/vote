extern crate vote;
extern crate safex;
use vote::utils::get_address_methods::get_omniwalletorg;
use vote::utils::get_address_methods::OmniList;
use vote::voting::poll_genesis::{PollRound, PollHash};

use safex::genesis::key_generation::KeyPair;

fn main() {

    let the_keys = KeyPair::create().unwrap();
    let omni_list = get_omniwalletorg(56);
    PollRound::new_wparams("hello".to_string(), 1, 2, vec!["hello".to_string(), "goodbye".to_string()], 3, the_keys, omni_list);

	//number 56 equates to Omni Smart Property #56 "SafeExchangeCoin"
	let the_list = get_omniwalletorg(56);
    let list_elements = the_list.return_list();

    print!("\n");
    let contains_or = the_list.check_existence("15N8mbsRwiwyQpsTUcGfETpStYkTFjcHvh".to_string());
	print!("Does the address contain? 15N8mbsRwiwyQpsTUcGfETpStYkTFjcHvh : {:?} \n", contains_or);

    let mut int_new = 0;

        for thethings in 0..list_elements.len() {
        	
            if list_elements[thethings].balance > 0 {
            	int_new += 1;
                println!("#{:?}", int_new);

                println!("address: {:?}", &list_elements[thethings].address);
                println!("safe exchange coin balance: {:?}", &list_elements[thethings].balance);
            }
        }

}
