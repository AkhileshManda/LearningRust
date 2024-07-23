use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //bind returns a new instance of the TcpListener
    //https://doc.rust-lang.org/std/net/struct.TcpListener.html
    //bind returns a Result<T,E> E(error) when binding fails
    //we use unwrap() to stop the program if error occurs

    /*About .unwrap()
    In Rust, when you have an operation that may either return a T or fail, you will have a value of type Result<T,E> or Option<T> (E will be the error condition in case of an interesting error).
    The function unwrap(self) -> T will give you the embedded T if there is one. If instead there is not a T but an E or None then it will panic.
    It is best used when you are positively sure that you don't have an error. If that is not the case usually it is better either pattern-match the error or use the try! macro ? operator to forward the error.
    */

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established");
    }
}
