// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_value = Some(String::from("rustlings"));
    /*
        if optional_value.is_some() {
            println!("the value of optional value is: {}", optional_value.unwrap());
        } else {
            println!("The optional value doesn't contain anything!");
        };
    */

    match optional_value {
        Some(x) => {
            println!("the value of optional value is: {}", x);
        }
        None => {
            println!("The optional value doesn't contain anything!");
        }
    }


    let mut optional_values_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_values_vec.push(Some(x));
    }

    // You can stack `Option<T>`'s into while let and if let
    while let Some(x) = optional_values_vec.pop() {
        println!("current value: {}", x.unwrap());
    }
}
