# Section IV - feeding the service

Now that the service that will be analyzing the data for `names`, we can start feeding the process.

In the terminal where you started the Kafka producer, start entering some names of people, \(John, Jonny, Jon, Johnathon, Jonathon\)

```text
ArchConfWorkshopUser:~/environment $ kafka_2.13-2.6.0/bin/kafka-console-producer.sh --topic names --bootstrap-server localhost:9092
>John
>Jonny
>Johny
>Jonathon
>Johnathon
>Jon
> 
```

In the terminal where the analyzer is running, you should see output notifying that it created the profile file and analyzed the data.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ ./target/debug/tdg_analyzer --topic names
Listening to the names topic ...
John
Creating new profile: names
Analyzed the data.
Jonny
Analyzed the data.
Johny
Analyzed the data.
Jonathon
Analyzed the data.
Johnathon
Analyzed the data.
Jon
Analyzed the data.
```

