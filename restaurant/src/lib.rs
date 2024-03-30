mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){
            println!("---add to waitlist---");
        }

        pub fn seat_at_table(){
            println!("---seat at table---");
        }
    }

    pub mod serving {
        pub fn take_order(){
            println!("---take order---");
        }

        pub fn serve_order(){
            println!("---serve order---");
        }

        pub fn take_payment(){
            println!("---take payment---");
        }
    }
}

use crate::front_of_house::serving;

pub fn serve_at_restaurant(){
    serving::take_order();
    serving::serve_order();
    serving::take_payment();
}

pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
}

//////////////////////////////////////////////////////
//
mod room1_of_house;

pub use crate::room1_of_house::room1;

pub fn playing_atroom1(){
    room1::play_at_room1();
}

//////////////////////////////////////////////////////
//
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn sub(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("---test---");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_workssub(){
        println!("---testsub---");
        let result = sub(5,2);
        assert_eq!(result,3);
    }

    #[test]
    fn test_restaurantmodules(){
        println!("---test restaurant modules---");
        eat_at_restaurant();

        serve_at_restaurant();

        playing_atroom1();
    }
}
