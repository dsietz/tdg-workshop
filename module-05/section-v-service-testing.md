# Section V - service testing



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
Notice that the test data generated is very limited to the data we entered:

```text
>John
>Jonny
>Johny
>Jonathon
>Johnathon
>Jon
```

 
{% endhint %}

