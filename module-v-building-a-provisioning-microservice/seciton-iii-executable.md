# Section III - executable

> [order\_clothing.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/bin/order_clothing.rs)

We're now ready to writing our executable: `src/bin/order_clothing.rs`.

We start by declaring our dependent external crates

```rust
extern crate daas;
extern crate kafka;
```

We then declare the modules we will be using.

```rust
use std::io;
use std::thread;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use daas::service::processor::{DaaSProcessor, DaaSProcessorMessage, DaaSProcessorService};
use std::sync::mpsc::{channel};
use serde_json::value::Value;
```

Next, we write the wrapper portion of the main function that will be called.

```rust
fn main() {
    std::env::set_var("RUST_LOG", "warn");
    env_logger::init();
}
```

We then add the configuration settings `hosts` and `topic` which used by the service to the main function.

> NOTE: Normally we would follow coding discipline and avoid hard coding configurations, \(e.g.: `hosts` and `topic` \). However, for this workshop we will skip this practice for the sake of time. If you'd like to implement command line arguments when starting the service, you can use the [`clap`](https://crates.io/crates/clap) crate to do so.

```rust
    // configuration settings
    let hosts = vec!("localhost:9092".to_string());
    let topic = "order.clothing".to_string();
```

The `DaaSProcessorService::start_listening` function takes four parameters that we need to prepare before calling it:

* kafka::consumer::Consumer
* std::sync::mpsc::Receiver
* Option for the Kafka client \(which we will set to `None`\)
* callback function 

```rust
    // parameters
    let (tx, rx) = channel();
    let consumer = Consumer::from_hosts(hosts)
                            .with_topic(topic.clone())
                            .with_fallback_offset(FetchOffset::Earliest)
                            .with_group(format!("{}-consumer", topic.clone()))
                            .with_offset_storage(GroupOffsetStorage::Kafka)
                            .create()
                            .unwrap();
```

Lastly, we add the section of the main function that will start the `DaaSProcessor`

However, since we want to first make sure everything is working correctly before adding any complex business logic, we will simply print the order number, store name, and order status to the console.

> NOTE: On line 7 we define what to do with the data and on line 8 we return `Ok(1)` notifying the provisioning of the DaaSDocument was successful.

```rust
    // start the processor
    let _handler = thread::spawn(move || {
        DaaSProcessor::start_listening(consumer, &rx, None, |msg: DaaSProcessorMessage, _none_var , _t: Option<&i8>|{
            let mut doc = msg.doc;
            let order: Value = serde_json::from_str(&String::from_utf8(doc.data_obj_as_ref().to_vec()).unwrap()).unwrap();

            println!("Order Number {} from the {} has a status of {}...", doc.source_uid, doc.source_name, order.get("status").unwrap());
            Ok(1)
        });
    });

    println!("Clothing Orders processor is running ...");
    println!("Press [Enter] to stop the Clothing Orders processor.");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            DaaSProcessor::stop_listening(&tx);
        }
        Err(error) => println!("error: {}", error),
    }
```

When we are finished, our `order_clothing` file should look like the following:

```rust
extern crate daas;
extern crate kafka;

use std::io;
use std::thread;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use daas::service::processor::{DaaSProcessor, DaaSProcessorMessage, DaaSProcessorService};
use std::sync::mpsc::{channel};
use serde_json::value::Value;


fn main() {
    std::env::set_var("RUST_LOG", "warn");
    env_logger::init();

    // configuration settings
    let hosts = vec!("localhost:9092".to_string());
    let topic = "order.clothing".to_string();

    // parameters
    let (tx, rx) = channel();
    let consumer = Consumer::from_hosts(hosts)
                            .with_topic(topic.clone())
                            .with_fallback_offset(FetchOffset::Earliest)
                            .with_group(format!("{}-consumer", topic.clone()))
                            .with_offset_storage(GroupOffsetStorage::Kafka)
                            .create()
                            .unwrap();

    // start the processor
    let _handler = thread::spawn(move || {
        DaaSProcessor::start_listening(consumer, &rx, None, |msg: DaaSProcessorMessage, _none_var , _t: Option<&i8>|{
            let mut doc = msg.doc;
            let order: Value = serde_json::from_str(&String::from_utf8(doc.data_obj_as_ref().to_vec()).unwrap()).unwrap();

            println!("Order Number {} from the {} has a status of {}...", doc.source_uid, doc.source_name, order.get("status").unwrap());
            Ok(1)
        });
    });

    println!("Clothing Orders processor is running ...");
    println!("Press [Enter] to stop the Clothing Orders processor.");

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => {
            DaaSProcessor::stop_listening(&tx);
        }
        Err(error) => println!("error: {}", error),
    }    
}
```

