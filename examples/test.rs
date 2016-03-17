extern crate vote;
extern crate safex;
use vote::utils::get_address_methods::get_omniwalletorg;



fn main() {

	//number 56 equates to Omni Smart Property #56 "SafeExchangeCoin"
	let the_list = get_omniwalletorg(56);
	let mut int_new = 0;

        for thethings in 0..the_list.len() {
        	
            if the_list[thethings].balance > 0 {
            	int_new += 1;
                println!("#{:?}", int_new);

                println!("address: {:?}", &the_list[thethings].address);
                println!("safe exchange coin balance: {:?}", &the_list[thethings].balance);
            }
        }

}
