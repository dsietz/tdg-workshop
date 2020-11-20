# Section V - service testing

### Retreiving Test Data

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

