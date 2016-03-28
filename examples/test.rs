extern crate vote;
extern crate safex;
use vote::utils::get_address_methods::get_omniwalletorg;
use vote::utils::get_address_methods::OmniList;
use vote::voting::poll_genesis::{PollRound, PollHash, PollPersona};
use vote::voting::vote_genesis::{VoteRound};
use vote::voting::validate_genesis::{VotingOutcome};

use safex::genesis::key_generation::KeyPair;

fn main() {
    //PollRound::make_poll();
    //VoteRound::form_vote();
    println!("{:?}", VotingOutcome::validate_outcome());





/*
    let the_keys = KeyPair::create().unwrap();
    let omni_list = get_omniwalletorg(56);
    let our_keys = PollPersona::import_keys();
    let keys = our_keys.return_keys();
    let our_poll = PollRound::new_wparams("hello".to_string(), 1, 2, vec!["hello".to_string(), "goodbye".to_string()], 3, &keys, omni_list);
    //our_poll.write_poll();

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
*/
}
