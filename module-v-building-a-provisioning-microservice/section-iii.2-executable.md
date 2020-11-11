# Section V - adding the business logic

> [order\_clothing.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/bin/order_clothing.rs)

Now that we have confirmed that the service is capturing and parsing the clothing order data correctly, we can add our business logic to the `main` function.

Let's first being by declaring some new uses

```rust
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde_json::json;
```

We will also be using a constant to define where our aggregated data records will be stored.

> Once again, this could be configured as a command line argument using the `clap` crate.

```rust
static WORKSPACE_LOCAL_STORAGE: &str = "./workshop_storage";
```

We next need to create some supportive functions _outside_ of the `main` function:

```rust
fn create_local_storage() {
    match fs::create_dir_all(WORKSPACE_LOCAL_STORAGE) {
        Ok(_) => {},
        Err(err) => {
            panic!("Warning: Could not create the local directory path {} : {:?}", WORKSPACE_LOCAL_STORAGE, err);
        },
    }
}
```

```rust
fn read_file(product_name: String) -> Option<Value> {
    let path = format!("{}/clothing-{}.json", WORKSPACE_LOCAL_STORAGE, product_name);
    let mut file = match File::open(path) {
        Ok(f) => {
            println!("Retreiving {} file", product_name);
            f
        },
        Err(_e) => return None,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Some(serde_json::from_str(&contents).unwrap())
}
```

```rust
fn save_file(product_name: String, content: String) -> std::io::Result<()>{
    let mut file = File::create(format!("{}/clothing-{}.json", WORKSPACE_LOCAL_STORAGE, product_name))?;
    file.write_all(content.as_bytes())
}
```

With all the `use` declarations and supportive functions in place, we can now start modifying the `main` function.

We first call the function to create the local storage directory when the service starts. This code can be added after the `parameters` section _within_ the `main` function.

```rust
    // Create the local storage directory for the aggregated data
    create_local_storage();
```

To add our business logic, \(inside the `callback` function after the `println` we were using to confirm the service is working correctly\) we add the following lines of code:

```rust
            match order.get("status").unwrap().as_str().unwrap() {
                "new" => {
                    let prd = order.get("product").unwrap().as_str().unwrap().replace(" ","_").replace("/","");
                    let qnty = order.get("quantity").unwrap().as_u64().unwrap();
                    let content = match read_file(prd.clone().to_string()){
                        Some(mut obj) => {
                            obj["orders"] = json!(obj["orders"].as_u64().unwrap() + qnty);
                            obj
                        },
                        None => {
                            let c = &format!("{{\"orders\":{:?}}}", qnty); 
                            println!("{}",c);
                            serde_json::from_str(c).unwrap()
                        },
                    };

                    match save_file(prd.clone().to_string(), content.to_string()) {
                        Ok(_ok) => Ok(1),
                        Err(err) => {
                            panic!("Warning: Could not save the clothing-{}.json file! : {:?}", prd, err);
                        },
                    }
                },
                _ => {
                    //do nothing
                    Ok(1)
                }
            }
```

The final state of your `order_clothing.rs` file should look like the following:

```rust
extern crate daas;
extern crate kafka;

use std::io;
use std::thread;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use daas::service::processor::{DaaSProcessor, DaaSProcessorMessage, DaaSProcessorService};
use std::sync::mpsc::{channel};
use serde_json::value::Value;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use serde_json::json;

static WORKSPACE_LOCAL_STORAGE: &str = "./workshop_storage";

fn create_local_storage() {
    match fs::create_dir_all(WORKSPACE_LOCAL_STORAGE) {
        Ok(_) => {},
        Err(err) => {
            panic!("Warning: Could not create the local directory path {} : {:?}", WORKSPACE_LOCAL_STORAGE, err);
        },
    }
}

fn read_file(product_name: String) -> Option<Value> {
    let path = format!("{}/clothing-{}.json", WORKSPACE_LOCAL_STORAGE, product_name);
    let mut file = match File::open(path) {
        Ok(f) => {
            println!("Retreiving {} file", product_name);
            f
        },
        Err(_e) => return None,
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Some(serde_json::from_str(&contents).unwrap())
}

fn save_file(product_name: String, content: String) -> std::io::Result<()>{
    let mut file = File::create(format!("{}/clothing-{}.json", WORKSPACE_LOCAL_STORAGE, product_name))?;
    file.write_all(content.as_bytes())
}


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

    // Create the local storage directory for the aggregated data
    create_local_storage();

    // start the processor
    let _handler = thread::spawn(move || {
        DaaSProcessor::start_listening(consumer, &rx, None, |msg: DaaSProcessorMessage, _none_var , _t: Option<&i8>|{
            let mut doc = msg.doc;
            let order: Value = serde_json::from_str(&String::from_utf8(doc.data_obj_as_ref().to_vec()).unwrap()).unwrap();

            println!("Order Number {} from the {} has a status of {}...", doc.source_uid, doc.source_name, order.get("status").unwrap());

            match order.get("status").unwrap().as_str().unwrap() {
                "new" => {
                    let prd = order.get("product").unwrap().as_str().unwrap().replace(" ","_").replace("/","");
                    let qnty = order.get("quantity").unwrap().as_u64().unwrap();
                    let content = match read_file(prd.clone().to_string()){
                        Some(mut obj) => {
                            obj["orders"] = json!(obj["orders"].as_u64().unwrap() + qnty);
                            obj
                        },
                        None => {
                            let c = &format!("{{\"orders\":{:?}}}", qnty); 
                            println!("{}",c);
                            serde_json::from_str(c).unwrap()
                        },
                    };

                    match save_file(prd.clone().to_string(), content.to_string()) {
                        Ok(_ok) => Ok(1),
                        Err(err) => {
                            panic!("Warning: Could not save the clothing-{}.json file! : {:?}", prd, err);
                        },
                    }
                },
                _ => {
                    //do nothing
                    Ok(1)
                }
            }
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

