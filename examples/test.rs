extern crate vote;
extern crate safex;
use vote::utils::get_address_methods::get_omniwalletorg;
use vote::utils::get_address_methods::OmniList;


fn main() {


	//number 56 equates to Omni Smart Property #56 "SafeExchangeCoin"
	let the_list = get_omniwalletorg(56);
    let list_elements = the_list.return_list();

    let contains_or = the_list.check_existence("15N8mbsRwiwyQpsTUcGfETpStYkTFjcHvh".to_string());
	print!("Does the address contain? : {:?} \n", contains_or);

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
