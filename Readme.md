google-merchant-client
--------------------

Sample usage:

```rust
extern crate google_merchant_client;
extern crate tokio_core;
use tokio_core::reactor::Core;
use google_merchant_client::{ZanoxClient,SearchType};

fn main(){   
	let mut core = Core::new().unwrap();
    let handle = core.handle();
	let client=GoogleMerchantClient::new(S,&handle);
	let work=client.get_feed(tring::from("FEED_URL").run();
	let products=core.run(work).unwrap();
}
```
