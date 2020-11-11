# Installing Tools

To help us get setup with the tools and services for this workshop, \(e.g.: Rust, Kafka\), we have built a setup script that will perform all the automated installations and configurations.

## Step 1

In the main terminal at the bottom where it states `ArchConfWorkshopUser:~/environment $` run the following commands - one at a time.

```text
aws s3 cp s3://iapp-archconf-workshop/workshop.sh workshop.sh
```

```text
sudo chmod +x workshop.sh
```

```text
./workshop.sh --workshop tdg
```

```text
source $HOME/.cargo/env
```

## Step 2

Let verify that Rust is installed correctly. Part of the `workshop` script was to create a `dummy` project, which should now appear as a directory in your left panel \(file tree\).

Let's run the `dummy` application.

```text
cd dummy && cargo run
```

You should see the following:

```text
ArchConfWorkshopUser:~/environment $ cd dummy && cargo run                                                                                                                                 
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/dummy`
Hello, world!
```

Let's change back the main directory.

```text
cd $HOME/environment
```

