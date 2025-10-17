fn main() {
    //! Data insertion -> Option<T> pattern match
    // let mut vec: Vec<i32> = Vec::new(); //? Collection type with <T> generics
    // vec.push(1);
    // vec.push(2);
    // vec.push(8);

    // let third: Option<&i32> = vec.get(2);

    // match third {
    //     Some(third) => println!("The third element is {third}"),
    //     None => println!("There is no third element"),
    // }

    //! Vector Macro
    // let v = vec![5, 6, 4]; //? Macro

    // let third: &i32 = &v[2];
    // println!("Third element {}", third);

    //! Ownership and borrowing
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];

    // v.push(6);
    // println!("The first element is : {first}");

    //? Here we can't do v.push(6); as v is in heap and memory are contiguous. first is borrowing the immutable value of v[0] which is the first value of vector. So if I add more value to v after it, the contiguous memory might get into trouble. Like if there is not enough space, the first value location will shift or deallocate to another memory which might cause dangling pointer error cause first is still pointing to the old memory.

    //! iteration in vector
    // let v = vec![100, 32, 57];

    // for i in &v {
    //     print!("{i} ");
    // }

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    /*     *i += 50;*/
    // }

    // for i in &v {
    //     print!("{i} ");
    // }

    //! Enum to store Multiple Types

    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // for cell in &row {
    //     match cell {
    //         SpreadsheetCell::Int(i) => println!("Int: {i}"),
    //         SpreadsheetCell::Float(f) => println!("Float: {f}"),
    //         SpreadsheetCell::Text(t) => println!("Text: {t}"),
    //     }
    // }
}
