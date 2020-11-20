extern crate kafka;
extern crate clap;

use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use clap::{Arg, App};
use std::path::Path;
use test_data_generation::{Profile, shared};

static WORKSPACE_LOCAL_STORAGE: &str = "../profiles";

fn make_consumer(data_topic: String) -> Result<Consumer, kafka::Error> {
    Consumer::from_hosts(vec!("localhost:9092".to_owned()))
      .with_topic(data_topic.to_owned())
      .with_fallback_offset(FetchOffset::Earliest)
      .with_group("my-group".to_owned())
      .with_offset_storage(GroupOffsetStorage::Kafka)
      .create()
}

fn analyse_data(topic: &str, data: &str) -> Result<bool, std::io::Error>{
  let profile_file = shared::string_to_static_str(format!("{}/{}", WORKSPACE_LOCAL_STORAGE, topic));
  let mut profile = match Path::new(&format!("{}/{}.json", WORKSPACE_LOCAL_STORAGE, topic)).exists() {
    true => {
      // use existing file
      Profile::from_file(&profile_file)
    },
    false => {
      // create new file
      println!("Creating new profile: {}",topic);
      Profile::new_with_id(topic.to_string())
    },
  };
  profile.analyze(&data);
  profile.pre_generate();
  profile.save(&profile_file)
}

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
    
    let mut consumer = match make_consumer(data_topic.to_string()) {
          Ok(c) => c,
          Err(err) => {
            panic!("{}",err);  
          },
      };
      
    loop {
      for ms in consumer.poll().unwrap().iter() {
        for m in ms.messages() {
          let data = String::from_utf8(m.value.to_vec()).unwrap();
          println!("{}", data);
          match analyse_data(data_topic, &data) {
            Ok(_) =>{
              //do nothing
              println!("Analyzed the data.");
            },
            Err(_err) => {
              println!("Unable to analyze the data for the topic {}!", data_topic);
            },
          }
        }
        consumer.consume_messageset(ms).unwrap();
      }
      consumer.commit_consumed().unwrap();
    }
}