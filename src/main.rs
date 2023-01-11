use crate::core::{
    HttpServer,
    Request,
    Response
};

pub mod core;


fn main(){
    let mut server = HttpServer::new(5000);
    server.route("/", root);
    server.listen();
}

fn root(_: &mut Request,res: &mut Response){
    res.send("hello world!");
}