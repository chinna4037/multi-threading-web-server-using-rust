use std::{
    fs,
    net::{TcpListener,TcpStream},
    io::{prelude::*,BufReader}, 
    thread,
    time::Duration,   
};
use web_server::ThreadPool;

fn main() {
    let listener=TcpListener::bind("127.0.0.1:7848").unwrap();
    println!("Server runing on port : 7848");
    let pool=ThreadPool::new(4);
    for stream in listener.incoming().take(2){
        pool.execute(||{
            handle_connection(stream.unwrap());
        });
    }
    println!("Hello, world!");
}

fn handle_connection(mut stream:TcpStream){
    let buf_reader=BufReader::new(&stream);
    // let _res:Vec<_>=buf_reader
    //                 .lines()
    //                 .map(| cur| cur.unwrap())
    //                 .take_while(|line| !line.is_empty())
    //                 .collect();

    let request_line=buf_reader.lines().next().unwrap().unwrap();
    let (status_line,file_name)=match &request_line[..]{
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK","index.html"),
        "GET /sleep HTTP/1.1" =>{
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK","index.html")
        }
        _ =>("HTTP/1.1 400 NOT FOUND","404.html"),
    };
    let contents=fs::read_to_string(file_name).unwrap();
    let length=contents.len();
    let response=format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    
}
