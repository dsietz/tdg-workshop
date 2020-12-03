# Section V - service testing

### Retrieving Test Data

We can now call the RESTful service to provide test data that is generated from the `profile` that is maintained by the Data Sample Analyzer microservice.

```text
./scripts/curl-tdg.sh names
```

```text
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jonn
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jon
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Johatton
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jonn
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Johatton
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jonny
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jon
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jonn
```

{% hint style="info" %}
Notice that the test data generated is very limited due to the data we entered:

```text
>John
>Jonny
>Johny
>Jonathon
>Johnathon
>Jon  
```

 The first 2 letters is always `Jo` and the 3rd character is either a `n` or a `h`
{% endhint %}

### Continuous Analyzing 

First, ensure that the Data Analyzer microservice for the `names` topic is still running. 

> `./target/debug/tdg_analyzer --topic names`

Using the terminal where the Kafka producer is still running for the `names` topic, start feeding it more names.

> `kafka_2.13-2.6.0/bin/kafka-console-producer.sh --topic names --bootstrap-server localhost:9092`

```text
>Chris
>Christopher
>Kris
>Kristy                   
>Christian  
>Christy  
```

{% hint style="info" %}
You should see messages in the Data Analyzer's consoler that the names are being processed.
{% endhint %}

```text
Analyzed the data.
Chris
Analyzed the data.
Christopher
Analyzed the data.
Kris
Analyzed the data.
Kristy
Analyzed the data.
Christian
Analyzed the data.
Christy
Analyzed the data.
```

Call the RESTful endpoint again and see what test data is now generated.

```text
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Crhas
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jhin
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Johny
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Chin
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jhin
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jon
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Johnathon
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jorny
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jris
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jornathan
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
Jhnashion
```



