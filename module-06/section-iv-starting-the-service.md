# Section IV - starting the service

We are now ready to start the RESTful service that will act as the data source for our DaaS platform.The web service will provide a JSON document\(s\) with the number of new orders by product.

 There are 2 ways to start the service.

1. Running using `cargo run` command while developing \(local service testing\)

> NOTE: we provide the argument `--bin myapp_reporting` because there are now multiple executables and must specify which one to run.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo run --bin myapp_reportinging
   Compiling rust-daas v0.1.0 (/home/ec2-user/environment/rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 11.71s
     Running `target/debug/myapp_sourcing`
```

To stop the service, use `ctrl` + `c`.

   2. Running using the executable.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo build
   Compiling rust-daas v0.1.0 (/home/ec2-user/environment/rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 7.23s
```

Whenever you use the `cargo build` command, it places the created executable in the target/debug directory with the same name that was defined in the `Cargo.toml` manifest.

Since it is an executable, simple run the executable from the command terminal.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ ./target/debug/myapp_reporting 
```

#### Checking the web service

Let's make sure the web service is working correctly by running the following script in an available terminal.

> Query number of orders for all products

```text
./scripts/curl-reporting.sh
```

```text
ArchConfWorkshopUser:~/environment $ ./scripts/curl-reporting.sh
all
```

> Query number of orders for the `leather jacket` product

```text
./scripts/curl-reporting.sh -p "leather jacket"
```

```text
ArchConfWorkshopUser:~/environment $ ./scripts/curl-reporting.sh -p "leather jacket"
leather_jacket
```



