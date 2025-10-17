fn main() {
    //! String as a collection
    // let mut s = String::new();

    // let data = "initial contents";
    // s = data.to_string(); //? to copy data

    // println!("{}", s);

    //! Updating String

    // let mut s = String::from("foo");
    // s.push_str(" bar");
    // s.push('s');
    // println!("{}", s);

    //? adding owned string instead of slice

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");
    // println!("s1 is {s1}");

    //! Concat String

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("{s}");

    //! Indexing                      

    // let hello = "Здравствуйте";

    // let s = &hello[0..3]; //? Byte wise storing so char can take up to one byte. That's why range work here

    // println!("{s}");
}
