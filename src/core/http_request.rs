use std::collections::HashMap;
pub struct Request{
    pub method: String,
    pub path: String,
    pub version: f32,
    headers: HashMap<String,String>,
    body: String
}

impl Request {
    pub(super) fn new(method: &str, path: &str, version: f32, headers: HashMap<String,String>)-> Request{
        Request { 
            method: String::from(method),
            path: String::from(path),
            version: version,
            headers: headers,
            body: "".to_string()
        }
    }
    pub(super) fn set_body(&mut self, body: &str){
        self.body = body.to_owned();
    }

    pub fn get_header(&self, name: &str)-> Option<&String>{
        self.headers.get(&name.to_string())
    }

    pub fn get_body(&self)-> &String{
        &self.body
    }
    
}