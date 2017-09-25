#[macro_use] extern crate serde_derive;
#[macro_use] extern crate log;
extern crate hyper;
extern crate serde;
extern crate serde_xml_rs;
extern crate chrono;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;
extern crate url;
pub mod structs;
use  tokio_core::reactor::Handle;
use structs::*;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use futures::future::Future;
use hyper::{Client,StatusCode};
use futures::Stream;
use url::Url;
use std::error::Error;
use std::fmt;

#[derive(Debug,Clone)]
pub struct GoogleMerchantClient{
 client:Client<HttpsConnector<HttpConnector>>,
}



#[derive(Debug, Clone)]
pub enum GoogleMerchantClientError{
	SendError(String),
	ResponseReadError(String),
	DeserializationError(String),
	ServerError(StatusCode,String)
}


impl fmt::Display for GoogleMerchantClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}",self)
    }
}

impl Error for GoogleMerchantClientError {
    fn description(&self) -> &str {
        "Zanox client error!"
    }

}

impl GoogleMerchantClient{
    pub fn new(handle:&Handle)->GoogleMerchantClient{    
        let client = hyper::Client::configure().connector(HttpsConnector::new(4, handle).unwrap()).build(handle);
         GoogleMerchantClient{
            client:client
        }
    }

    pub fn get_feed(&self,feed:String)->ZanoxAPIRequest{
        ZanoxAPIRequest{
            client:self,
            url:Url::parse(&feed).unwrap()
        }
    }

    fn get_products_using(&self,url:String)->Box<Future<Item=Feed, Error=GoogleMerchantClientError>>{
        info!("Fetching products using URL {:?}",url);
        let uri = url.parse::<hyper::Uri>().unwrap();

       let out=self.client.get(uri).map_err(|e|{
            GoogleMerchantClientError::SendError(format!("{}",e))
        }).and_then(|r|{
            let status:StatusCode=r.status();
            r.body().concat2().map_err(|e|{
                GoogleMerchantClientError::ResponseReadError(format!("{}",e))
            }).map(move |d|{(status,d)})
        }).and_then(|(status,data)|{
           String::from_utf8((&data).to_vec()).map_err(|e|{
                GoogleMerchantClientError::ResponseReadError(format!("{}",e))
            }).map(move |str|{
                (status,str)
            })
        }).and_then(|(status,str)|{
            match status{
                StatusCode::Ok => Ok(str),
                _ => {
                    Err(GoogleMerchantClientError::ServerError(status,str))
                }
            }
        }).and_then(|str|{
             let out:Result<Feed,GoogleMerchantClientError>=serde_xml_rs::deserialize(str.as_bytes()).map_err(|e|{
                GoogleMerchantClientError::DeserializationError(format!("{} {:?}",e,str))
            });
             out
        });
        Box::new(out)
    }
}

#[derive(Debug, Clone)]
pub struct ZanoxAPIRequest<'a>{
    client:&'a GoogleMerchantClient,
    url:Url
}

impl <'a> ZanoxAPIRequest<'a>{
    pub fn run(self)->Box<Future<Item=Feed, Error=GoogleMerchantClientError>>{
        self.client.get_products_using(self.url.into_string())
    }
}