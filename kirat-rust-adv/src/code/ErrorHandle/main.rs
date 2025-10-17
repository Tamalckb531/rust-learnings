use std::fs::File;
fn main() {
    //! Unrecoverable Error with panic !
    // let v = vec![1, 2, 3];
    //v[100];
    //? This will panic the main thread and then do backtrace to delete all the function from stack memory, to free up the memory

    //! Recoverable Errors with Result
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {error:?}"),
    };
}
