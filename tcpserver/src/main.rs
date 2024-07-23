use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

/*About BufReader
struct adds buffering to any reader.
BufReader<R> can improve the speed of programs that make small and repeated read calls to the same file or network socket. It does not help when reading very large amounts at once, or reading just one or a few times. It also provides no advantage when reading from a source that is already in memory, like a Vec<u8>.
*/

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

        handle_connection(stream);

        // println!("Connection established");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // if request_line == "GET / HTTP/1.1" {
    //     // let response = "HTTP/1.1 200 OK\r\n\r\n";
    //     let status_line = "HTTP/1.1 200 OK";
    //     let contents = fs::read_to_string("helloworld.html").unwrap();
    //     let length = contents.len();

    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //     stream.write_all(response.as_bytes()).unwrap(); //normally we would have to add error handling here
    // } else {
    //     //some other req
    //     let status_line = "HTTP/1.1 404 NOT FOUND";
    //     let contents = fs::read_to_string("404.html").unwrap();
    //     let length = contents.len();

    //     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    //     stream.write_all(response.as_bytes()).unwrap();
    // }
    //
    // Simpler refactoring of above code

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "helloworld.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();

    // println!("Request: {http_request:#?}");
}

/*
Sometimes, you’ll see multiple messages printed for one browser request; the reason might be that the browser is making a request for the page as well as a request for other resources, like the favicon.ico icon that appears in the browser tab.
When stream goes out of scope and is dropped at the end of the loop, the connection is closed as part of the drop implementation. Browsers sometimes deal with closed connections by retrying, because the problem might be temporary. The important factor is that we’ve successfully gotten a handle to a TCP connection!
*/
