
extern crate google_merchant_client;
extern crate tokio_core;
extern crate pretty_env_logger;
use tokio_core::reactor::Core;
use google_merchant_client::{GoogleMerchantClient};

const FEED_URL:&'static str="https://raw.githubusercontent.com/iambotHQ/google-merchant-client/master/tests/feed.xml"; 

#[test]
fn should_download_products(){   
	pretty_env_logger::init().unwrap();
	let mut core = Core::new().unwrap();
    let handle = core.handle();
	let client=GoogleMerchantClient::new(&handle);
	let work=client.get_feed(String::from(FEED_URL)).run();
	let products=core.run(work).unwrap();
	assert_eq!(products.channels[0].items.len(),2);
}