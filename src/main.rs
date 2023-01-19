use std::{env, path::Path};

use crate::core::{
    HttpServer,
    Request,
    Response
};

pub mod core;


fn main(){
    let mut server = HttpServer::new(5000);
    server.route("/", root);
    server.route("/file", file);
    server.listen();
}

fn root(_: &mut Request,res: &mut Response){
    res.send("hello world!");
}

fn file(_: &mut Request,res: &mut Response){
    let path = env::current_dir().unwrap();
    res.file(path.join("public/text.html").to_str().unwrap());
}