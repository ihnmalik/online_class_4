mod practice;
mod new;
use new::class_code;

mod restaurant;

// use restaurant::eat_at_restaurant;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn main() {
    println!("Hello, world!");

    println!("changes");

    practice::run();
    let some_num = practice::another_run(54);
    class_code::new();

//    eat_at_restaurant();

    println!("some_num {}", some_num);

     // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}



// mod front_of_house {
//     mod hosting {
//         fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }