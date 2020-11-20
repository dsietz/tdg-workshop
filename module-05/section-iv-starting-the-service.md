# Section IV - starting the service

To start the RESTful service, run the `cargo run --bin tdg_service` command.

{% hint style="info" %}
Alternatively, you can perform a `cargo build` and then run the executable directly `./target/debug/tdg_service`
{% endhint %}

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ cargo run --bin tdg_service
    Finished dev [unoptimized + debuginfo] target(s) in 2.55s
     Running `target/debug/tdg_service`
```

We can now call the RESTful service to provide test data that is generated from the `profile` that is maintained by the Data Sample Analyzer microservice.

```text
./scripts/curl-tdg.sh names
```

```text
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
JohnA
rchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jonn
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jonn
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
John
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
John
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jonn
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
John
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
```

## STOPPED HERE

