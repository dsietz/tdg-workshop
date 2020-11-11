# Section III - executable

> genesis.rs

Since the SDKs contain all the modules we will need for our web service, we can go right to writing our executable: `src/bin/genesis.rs`.

We start by declaring our dependent external crates

```rust
extern crate daas;
extern crate kafka;
extern crate rusoto_core;
```

We then declare the modules we will be using.

```rust
use std::io;
use rusoto_core::Region;
use kafka::consumer::{FetchOffset, GroupOffsetStorage};
use daas::service::processor::{DaasGenesisProcessor, DaaSGenesisProcessorService};
use daas::storage::s3::{S3BucketManager, S3BucketMngr};
```

We need to provide a constant and a supportive function.

```rust
// NOTE: Modify the Bucket name to match your bucket
// Credentials are read from the environment variables AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY
pub const BUCKET_NAME: &'static str = "iapp-archconf-workshop";

fn get_bucket() -> S3BucketMngr {
    S3BucketMngr::new(Region::UsEast1, BUCKET_NAME.to_string())
}
```

Finally, we write the main function that will be called.

```rust
fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let hosts = vec!("localhost:9092".to_string());

    let stopper = DaasGenesisProcessor::run(hosts, FetchOffset::Earliest, GroupOffsetStorage::Kafka, get_bucket());

    println!("Genesis processor is running ...");
    println!("Press [Enter] to stop the Genesis processor.");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            DaasGenesisProcessor::stop(stopper);
        }
        Err(error) => println!("error: {}", error),
    }    
}
```

When you are finished, the `genesis.rs` file should look like this:

```rust
extern crate daas;
extern crate kafka;
extern crate rusoto_core;

use std::io;
use rusoto_core::Region;
use kafka::consumer::{FetchOffset, GroupOffsetStorage};
use daas::service::processor::{DaasGenesisProcessor, DaaSGenesisProcessorService};
use daas::storage::s3::{S3BucketManager, S3BucketMngr};

// NOTE: Modify the Bucket name to match your bucket
// Credentials are read from the environment variables AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY
pub const BUCKET_NAME: &'static str = "iapp-archconf-workshop";

fn get_bucket() -> S3BucketMngr {
    S3BucketMngr::new(Region::UsEast1, BUCKET_NAME.to_string())
}

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let hosts = vec!("localhost:9092".to_string());

    let stopper = DaasGenesisProcessor::run(hosts, FetchOffset::Earliest, GroupOffsetStorage::Kafka, get_bucket());

    println!("Genesis processor is running ...");
    println!("Press [Enter] to stop the Genesis processor.");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            DaasGenesisProcessor::stop(stopper);
        }
        Err(error) => println!("error: {}", error),
    }    
}
```

