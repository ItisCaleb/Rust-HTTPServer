use std::collections::HashMap;
use crate::core::HttpError;
use crate::core::HttpStatus;

pub(super) fn parse_headers(raw_request: &[String])-> HashMap<String,String>{
    let mut headers:HashMap<String,String> = HashMap::new();
    for line in raw_request{
        let header = match line.split_once(':') {
            Some(v)=>v,
            None=>continue
        };
        headers.insert(header.0.trim().to_string(), header.1.trim().to_string());
    }
    headers
}

pub(super) fn parse_first_line(line: &String)-> Result<(String,String,f32),HttpError> {
    let line: Vec<_> = line.split(" ").collect();
    if line.len()!=3{
        return Err(HttpError::bad_request())
    }
    let version = line[2];
    if version.len()<6 || &version[0..5]!="HTTP/"{
        return Err(HttpError::bad_request());
    }
    let version: f32 = match version[5..].parse::<f32>() {
        Ok(v)=> v,
        Err(_)=> return Err(HttpError::bad_request())
    }; 
    if version >= 2.0{
        return Err(HttpError::new(HttpStatus::NotImplemented, "meh"));
    }
    let path:String = match normalize_path(line[1]) {
        Ok(v)=>v,
        Err(err)=> return  Err(err)
    };
    Ok((line[0].to_string(),path,version))
}

fn normalize_path(path: &str)-> Result<String,HttpError>{
    let decoded:String = match url_decode(path) {
        Ok(v)=>v,
        Err(err)=>return Err(err)
    };
    let mut result = "/".to_owned();
    let mut stack:Vec<String> = vec![];
    for s in decoded.split('/'){
        match s {
            ".."=>{stack.pop();},
            "."=>{},
            ""=>{},
            other=> stack.push(other.to_string())
        }
    }
    for seg in stack{
        result.push_str(&seg);
        result.push('/');
    }
    Ok(result)
}

fn url_decode(url: &str)-> Result<String,HttpError>{
    let mut result: String = "".to_owned();
    let mut i=0;
    while i<url.len() {
        let mut word = url.chars().nth(i).unwrap();
        if word == '%'{
            if i+3 > url.len(){
                return Err(HttpError::bad_request()); 
            }
            word = hex::decode(&url[i+1..i+3])
                .expect("Decode error")[0] as char;
            i+=2;
        }
        i+=1;
        result.push(word);
    }
    Ok(result)
}
