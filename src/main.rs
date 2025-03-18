use std::{ fs, io::{ prelude::*, BufReader }, net::{ TcpListener, TcpStream } };
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    if http_request.is_empty() {
        return;
    }

    let (status_line, contents) = get_page_by_request(&http_request[0]);
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_page_by_request(request: &String) -> (&str, String) {
    if request.starts_with("GET / ") {
        return ("HTTP/1.1 200 OK", fs::read_to_string("pages/hello.html").unwrap());
    }
    return ("HTTP/1.1 404 NOT FOUND", fs::read_to_string("pages/404.html").unwrap());
}
