# Section IV - starting the service

We are now ready to start the microservice that will listen for data on the `genesis` topic and stash it in the S3 bucket, then send it downstream to the next topic which is dynamically built based on the metadata of the DaaSDocument.

 There are 2 ways to start the service.

1. Running using `cargo run` command while developing \(local service testing\)

> IMPORTANT: Run the executable in a new terminal so that you can have the sourcing and the genesis services running in parallel.

> NOTE: we provide the argument `--bin myapp_genesisg` because there are now multiple executables and must specify which one to run.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo run --bin myapp_genesis
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/myapp_genesis`
Genesis processor is running ...
Press [Enter] to stop the Genesis processor.
[2020-11-04T21:28:23Z INFO  daas::service::processor] Putting document order~clothing~iStore~5000 in S3
[2020-11-04T21:28:23Z INFO  daas::service::processor] Brokering document order~clothing~iStore~5000 ... 
[2020-11-04T21:28:27Z INFO  daas::service::processor] Putting document order~clothing~iStore~5000 in S3
[2020-11-04T21:28:27Z INFO  daas::service::processor] Brokering document order~clothing~iStore~5000 ... 
```

To stop the service, use `ctrl` + `c`.

   2. Running using the executable.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo build
   Compiling kafka v0.8.0
   Compiling daas v0.2.0
   Compiling rust-daas v0.1.0 (/home/ec2-user/environment/rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 26.49s
```

Whenever you use the `cargo build` command, it places the created executable in the target/debug directory with the same name that was defined in the `Cargo.toml` manifest.

Since it is an executable, simple run the executable from the command terminal.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ ./target/debug/myapp_genesis 
```

