# hello-advpro25

### Commit 1 notes 


```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    println!("Request: {:#?}", http_request);
}
```

`BufReader::new(&mut stream)` wraps the stream in a BufReader to efficiently read from it. We then use several methods to process the BufReader. 
- `.lines()` returns an iterator over the lines of the stream.
- `.map(|result| result.unwrap())` unwraps each line, panicking if an error occours.
- `.take_while(|line| !line.is_empty())` takes lines until an empty line is encountered indicating the end of the HTTP request headers.
- `.collect()` collects the line to a `vec`. 

then `println!("Request: {:#?}", http_request)` prints the collected HTTP request lines to the console.

this code reads HTTP request headers from each connection and prints them to the console.

### Commit 2 notes 
```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length:
   {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
```

After the code reads HTTP request headers, the code sends a response to the client containing the response header `status_line` then `Content-Length`, after that the `contents` which reads the `hello.html` file. 

![image](./image.png)