extern crate vote;

use vote::utils::get_address_methods::get_omniwalletorg;


fn main() {


	let the_list = get_omniwalletorg(56);
	let mut int_new = 0;

        for thethings in 0..the_list.len() {
        	
            if the_list[thethings].balance > 0 {
            	int_new += 1;
                println!("#{:?}", int_new);

                println!("address: {:?}", &the_list[thethings].address);
                println!("maidsafe coin balance: {:?}", &the_list[thethings].balance);
            }
        }

}