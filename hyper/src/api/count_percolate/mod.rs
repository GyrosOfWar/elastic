//! http://www.elastic.co/guide/en/elasticsearch/reference/master/search-percolate.html

//Autogenerated

use hyper::client::Client;
use hyper::header::{Headers, ContentType};
use hyper::client::response::Response;
use hyper::error::Result;

pub fn post_index_type_id<'a>(client: &'a mut Client, base: &'a str,
                          index: &'a str, _type: &'a str, id: &'a str,
                          body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 1 + 17 + index.len() +
                                  _type.len() + id.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/");
    url_fmtd.push_str(id);
    url_fmtd.push_str("/_percolate/count");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn post_index_type<'a>(client: &'a mut Client, base: &'a str, index: &'a str,
                       _type: &'a str, body: String) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 17 + index.len() +
                                  _type.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_percolate/count");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.post(&url_fmtd).headers(headers).body(&body);
    res.send()
}
pub fn get_index_type_id<'a>(client: &'a mut Client, base: &'a str,
                         index: &'a str, _type: &'a str, id: &'a str)
 -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 1 + 17 + index.len() +
                                  _type.len() + id.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/");
    url_fmtd.push_str(id);
    url_fmtd.push_str("/_percolate/count");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}
pub fn get_index_type<'a>(client: &'a mut Client, base: &'a str, index: &'a str,
                      _type: &'a str) -> Result<Response>{
    let mut url_fmtd =
        String::with_capacity(base.len() + 1 + 1 + 17 + index.len() +
                                  _type.len());
    url_fmtd.push_str(base);
    url_fmtd.push_str("/");
    url_fmtd.push_str(index);
    url_fmtd.push_str("/");
    url_fmtd.push_str(_type);
    url_fmtd.push_str("/_percolate/count");
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    let res = client.get(&url_fmtd).headers(headers);
    res.send()
}