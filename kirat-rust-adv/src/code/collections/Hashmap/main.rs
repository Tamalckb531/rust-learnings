use std::collections::HashMap;
fn main() {
    //! Creation of hashmap
    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    //? Get value
    // let team_name = String::from("Yellow");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    // println!("{score}");

    //? Get value in loop way
    // for (key, value) in &scores {
    //     println!("{key} : {value}");
    // }

    //! Entry API : First check a value exist or not. If not then insert the value. Otherwise the prev remains

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{scores:?}");

    //! Updating a Value Based on the Old Value

    // let text = "No need to run and hide, it's a wonderful wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{map:?}");
}
