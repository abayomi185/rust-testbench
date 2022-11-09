mod style;
mod utils;

use crate::style::apply_style;
use ::std::collections::HashMap;

// Use this when mod for this component is not used
use basics::utils::auth;
// Otherwise this works
// use crate::utils::auth;

struct UserData {
    fname: String,
    lname: String,
    age: i32,
    height: f32,
    active_user: bool,
}

impl UserData {
    // UUser
    pub fn get_user_age(&self) -> i32 {
        self.age
    }
}

impl ActiveStatus for UserData {
    fn is_active(&self) -> &str {
        if self.active_user {
            "User is active"
        } else {
            "User is NOT active"
        }
    }
}

struct GroupData {
    id: i32,
    name: String,
    no_of_members: i32,
    active_group: bool,
}

// Kinda like abstract classes
pub trait ActiveStatus {
    // This can also have a default implementation
    fn is_active(&self) -> &str;

    // Default implementation of is_active
    // fn is_active(&self) -> &str {
    //     format!("This is the default implementation")
    // }
}

fn main() {
    type CustomTypeAlias = i32;
    let random_number: CustomTypeAlias = 10;

    let mut dict = HashMap::new();
    dict.insert(1, "one");

    println!("{:#?}", dict);

    let value_to_get = 1;
    // Output is option<&str> - Some value or None
    let value_option_gotten = dict.get(&value_to_get);

    println!("{:?}", value_option_gotten);
    // Safe handling
    if let Some(value_gotten) = value_option_gotten {
        println!("{:?}", value_gotten);
    }

    apply_style();
}
