# Section IV - starting the service

### Produce to the Kafka Topic

{% hint style="info" %}
NOTE: Make sure that Zookeeper and Kafka are running.
{% endhint %}

In a new terminal, start up a Kafka producer for the specified topic - in this case `names`.

```text
kafka_2.13-2.6.0/bin/kafka-console-producer.sh --topic names --bootstrap-server localhost:9092
```

## STOPPED HERE

We are now ready to start the RESTful service that will listen for data that needs to be sourced and feeds it to the event flow.

There are 2 ways to start the service.

1. Running using `cargo run` command while developing \(local service testing\)

> NOTE: we provide the argument `--bin myapp_sourcing` because there are now multiple executables and must specify which one to run.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo run --bin myapp_sourcing
   Compiling rust-daas v0.1.0 (/home/ec2-user/environment/rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 11.71s
     Running `target/debug/myapp_sourcing`
```

To stop the service, use `ctrl` + `c`.

1. Running using the executable.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo build
   Compiling rust-daas v0.1.0 (/home/ec2-user/environment/rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 7.23s
```

Whenever you use the `cargo build` command, it places the created executable in the target/debug directory with the same name that was defined in the `Cargo.toml` manifest.

Since it is an executable, simple run the executable from the command terminal.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ ./target/debug/myapp_sourcing
```

