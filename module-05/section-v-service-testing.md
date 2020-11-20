# Section V - service testing

### Retrieving Test Data

We can now call the RESTful service to provide test data that is generated from the `profile` that is maintained by the Data Sample Analyzer microservice.

```text
./scripts/curl-tdg.sh names
```

```text
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
John
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh names
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

 The average length is 4 characters and the first 2 letters is always `Jo` and the 3rd character is either a `n` or a `h`
{% endhint %}

### Continuous Analyzing 

First, ensure that the Data Analyzer microservice for the `names` topic is still running. 

> `./target/debug/tdg_analyzer --topic names`

Using the terminal where the Kafka producer is still running for the `names` topic, start feeding it more names.

> `kafka_2.13-2.6.0/bin/kafka-console-producer.sh --topic names --bootstrap-server localhost:9092`

{% hint style="info" %}
You should see messages in the Data Analyzer's consoler that the names are being processed.
{% endhint %}

Call the RESTful endpoint again and see what test data is now generated.



