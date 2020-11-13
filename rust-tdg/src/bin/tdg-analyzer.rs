extern crate kafka;
extern crate clap;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use clap::{Arg, App};



fn main() {
    let matches = App::new("Sample Data Analyzer")
                          .arg(Arg::with_name("topic")
                               .help("Specify the Kafak topic to pull data from")
                               .required(true)
                               .short("t")
                               .long("topic")
                               .takes_value(true))
                          .get_matches();

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let data_topic = matches.value_of("topic").unwrap();
    println!("Listening to the {} topic ...", data_topic);
    
    let mut consumer = match Consumer::from_hosts(vec!("localhost:9092".to_owned()))
      .with_topic(data_topic.to_owned())
      .with_fallback_offset(FetchOffset::Earliest)
      .with_group("my-group".to_owned())
      .with_offset_storage(GroupOffsetStorage::Kafka)
      .create() {
          Ok(c) => c,
          Err(err) => {
            panic!("{}",err);  
          },
      };
      
    loop {
      for ms in consumer.poll().unwrap().iter() {
        for m in ms.messages() {
          println!("{}", String::from_utf8(m.value.to_vec()).unwrap());
        }
        consumer.consume_messageset(ms);
      }
      consumer.commit_consumed().unwrap();
    }
}