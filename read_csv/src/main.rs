use csv;
use std::error::Error;

fn main() {
    if let Err(e) = read_from_file("./test.csv"){
        eprintln!("{}",e);
    }
}

fn read_from_file(path: &str) -> Result<(), Box<dyn Error>>{
    //Using the ? Operator:
    // The ? operator is used to handle the Result returned by csv::Reader::from_path(path). Here's how it works:
    // If csv::Reader::from_path(path) returns Ok(reader), the ? operator will unwrap the Ok value and assign the reader to the let mut reader variable.
    // If csv::Reader::from_path(path) returns Err(e), the ? operator will return the error from the current function. 
    let mut reader = csv::Reader::from_path(path)?;
    
    for result in reader.records(){
        let record = result? ;
        
        println!("{:?}", record);
    }
    Ok(())
}
