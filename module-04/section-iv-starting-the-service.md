# Section III - starting the service

### Produce to the Kafka Topic

{% hint style="info" %}
NOTE: Make sure that Zookeeper and Kafka are running.
{% endhint %}

In a new terminal, start up a Kafka producer for the specified topic - in this case `names`.

```text
kafka_2.13-2.6.0/bin/kafka-console-producer.sh --topic names --bootstrap-server localhost:9092
```

We are now ready to start the microservice service that will listen for data that needs to be analyzed and added to the profile's algorithm.

There are 2 ways to start the service.

1. Running using `cargo run` command while developing \(used for local service testing\)

> NOTE: we provide the argument `--bin myapp_sourcing` because there are now multiple executables and must specify which one to run.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cargo run --bin tdg_analyzer -- --topic names
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target/debug/tdg_analyzer --topic names`
Listening to the names topic ...
```

To stop the service, use `ctrl` + `c`.

1. Running using the executable.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cargo build
```

Whenever you use the `cargo build` command, it places the created executable in the target/debug directory with the same name that was defined in the `Cargo.toml` manifest.

Since it is an executable, simple run the executable from the command terminal.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ ./target/debug/tdg_analyzer --topic names
```

