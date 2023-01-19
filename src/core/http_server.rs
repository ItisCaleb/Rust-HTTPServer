use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    collections::HashMap,
    thread,
    str,
    sync::Arc
};
use strum::EnumProperty;

use crate::core::{
    Request,
    Response,
    http_request_parser::{parse_headers, parse_first_line},
    HttpError
};


pub struct HttpServer{
    port: u32,
    handlers: HashMap<String,fn(&mut Request,&mut Response)>
}

impl HttpServer {
    pub fn new(port: u32) -> HttpServer{
        HttpServer{
            port:port,
            handlers: HashMap::new()
        }
    }

    pub fn route(&mut self,path: &str, handler: fn(&mut Request,&mut Response)){
        self.handlers.insert(String::from(path), handler);
    }

    pub fn listen(self){
        let host = format!("127.0.0.1:{}",self.port);
        let listener = TcpListener::bind(host).unwrap(); 
        let shared_self = Arc::new(self);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let server = shared_self.clone();
            thread::spawn(move||{
                let stream = stream;
                let (mut req,mut res) = match handle_request(&stream) {
                    Ok(v)=>v,
                    Err(mut res)=> {
                        send_response(&stream, &mut res,true);
                        return;
                    }
                };
                match &server.handlers.get(&req.path){
                    Some(f)=> f(&mut req,&mut res),
                    None=> error_response(&mut res,HttpError::not_found())
                }
                let send_body = &req.method != "HEAD";
                send_response(&stream,&mut res,send_body);
            });
        }
    }
}

fn handle_request(mut stream: &TcpStream)-> Result<(Request,Response),Response>{
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buf = String::new();
    //first we read first line and headers
    loop{
        buf_reader.read_line(&mut buf).unwrap();
        if buf.len()>4 && &buf[buf.len()-4..] == "\r\n\r\n"{
            break;
        }
    }
    let request:Vec<_> = buf.split("\r\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    println!("{:#?}",request);
    let mut res = Response::new();
    let (method, path, version) = match parse_first_line(&request[0]) {
        Ok(v)=>v,
        Err(err)=> {
            error_response(&mut res, err);
            return Err(res)
        }
    };
    let mut req = Request::new(
        &method,
        &path,
        version,
        parse_headers(&request[1..])
    );
    let body:String = match read_body(&mut buf_reader,&req) {
        Ok(v)=>v,
        Err(err)=>{
            error_response(&mut res, err);
            return Err(res)
        }
    };
    req.set_body(&body);
    println!("{}",&body);
    println!("Request: {:#?}",request);
    Ok((req,res))
}

fn send_response(mut stream: &TcpStream, res: &mut Response,send_body:bool){
    let status = &res.status;
    let status_code = status.get_str("code").unwrap();
    let status_msg = status.get_str("msg").unwrap();
    let mut response: String = format!("HTTP/{} {} {}\r\n",res.version,
    status_code,status_msg);
    response.push_str(&format!("{}\r\n"
        ,&res.format_header()));
    if send_body{
        response.push_str(&res.get_content());
    }
    stream.write_all(response.as_bytes()).unwrap();
}

fn error_response(res: &mut Response, err: HttpError){
    res.set_header("Connection", "close")
        .status(err.status)
        .send(&err.message);
}

fn read_body(buf_reader: &mut BufReader<&mut &TcpStream>, req: &Request)->Result<String, HttpError>{
    match req.get_header("Content-Length") {
        Some(len)=>{
            let len = match len.parse::<usize>() {
                Ok(v)=>v,
                Err(_)=> return Err(HttpError::bad_request())
            };
            let mut body = vec![0u8;len];
            match buf_reader.read_exact(&mut body) {
                Ok(_)=>(),
                Err(_)=> return Err(HttpError::bad_request()),
            };
            let body = match str::from_utf8(&body) {
                Ok(v)=>v,
                Err(_)=> return Err(HttpError::bad_request())
            };
            Ok(body.to_owned())
        },
        None=>Ok("".to_string())
    }
}