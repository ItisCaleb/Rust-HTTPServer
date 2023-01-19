use std::collections::HashMap;
use chrono::Local;
use crate::core::HttpStatus;
use std::{
    fs,
    path::Path
};
use phf::phf_map;

pub struct Response{
    pub(super) status: HttpStatus,
    pub(super) version: f32,
    headers: HashMap<String,String>,
    body: Option<String>
}

static MIMETYPE: phf::Map<&'static str, &'static str> = 
    phf_map! {
        "html"=>"text/html",
        "css"=>"text/css",
        "js"=>"text/javascript",
        "png"=>"image/png",
        "jpg"=>"image/jpeg",
        "gif"=>"image/gif",
        "webp"=>"image/webp",
        "svg"=>"image/svg+xml"
    };


impl Response {
    pub(super) fn new()-> Response{
        let mut res = Response { 
            status: HttpStatus::OK,
            version: 1.1,
            headers: HashMap::new(),
            body: Option::None
        };
        let date = Local::now().format("%a, %d %b %Y %X GMT");
        res.set_header("Server", "ItisCaleb/1.0")
            .set_header("Date",  &date.to_string());
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

    pub fn set_header(&mut self, name: &str, value: &str)-> &mut Response{
        if  name.chars().any(|c| c.is_control()) ||
            value.chars().any(|c| c.is_control()) {
            println!("No control character allowed in header");
            return self;
        }
        self.headers.insert(name.to_string(), value.to_string());
        self
    }

    pub fn send(&mut self, content: &str)-> &mut Response{
        self.body = Some(String::from(content));
        self.set_header("Content-Length", &content.chars().count().to_string());
        self
    }
    pub fn status(&mut self, status: HttpStatus)-> &mut Response{
        self.status = status;
        self
    }

    pub fn html(&mut self, path: &str)-> &mut Response{
        let file = fs::read_to_string(path)
            .unwrap_or("File Not Found".to_string());
        self.set_header("Content-Length", &file.chars().count().to_string());
        self.set_header("Content-Type", "text/html");
        self.body = Some(file);
        self
    }

    pub fn file(&mut self, path: &str)-> &mut Response{
        let (file,mime) = match fs::read_to_string(path) {
            Ok(content)=>
                (content,Path::new(path).extension().and_then(|ext| ext.to_str())),
            _=> (String::from("File Not Found"),Some("text"))
            
        };
        self.set_header("Content-Length", &file.chars().count().to_string());
        let mime = match mime {
            Some(ext)=>MIMETYPE.get(ext).unwrap_or(&"text/plain"),
            _=>"text/plain"
        };
        self.set_header("Content-Type", mime);
        self.body = Some(file);
        self
    }

}