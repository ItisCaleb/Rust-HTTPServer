use std::collections::HashMap;
use chrono::Local;
use crate::core::HttpStatus;
use std::fs;

pub struct Response{
    pub(super) status: HttpStatus,
    pub(super) version: f32,
    headers: HashMap<String,String>,
    body: Option<String>
}

impl Response {
    pub(super) fn new()-> Response{
        let mut res = Response { 
            status: HttpStatus::OK,
            version: 1.1,
            headers: HashMap::new(),
            body: Option::None
        };
        let date = Local::now().format("%a, %d %b %Y %X GMT");
        res.add_header("Server", "ItisCaleb/1.0")
            .add_header("Date",  &date.to_string());
        res
    }

    pub(super) fn format_header(&mut self)-> String{
        let headers = self.headers.iter()
            .map(|(key,value)| 
                format!("{}: {}\r\n",key.trim(),value))
            .collect();
        headers
    }

    pub(super) fn get_content(&mut self)-> String{
        match &self.body {
            Some(content)=>content.to_string(),
            None=> "".to_string()
        }
    }

    
    pub fn get_header(&mut self, name: &str)-> Option<&String>{
        self.headers.get(&name.to_string())
    }

    pub fn add_header(&mut self, name: &str, value: &str)-> &mut Response{
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn send(&mut self, content: &str)-> &mut Response{
        self.body = Some(String::from(content));
        self.add_header("Content-Length", &content.chars().count().to_string());
        self
    }
    pub fn status(&mut self, status: HttpStatus)-> &mut Response{
        self.status = status;
        self
    }

    pub fn html(&mut self, path: &str)-> &mut Response{
        //self.body;
        self
    }

}