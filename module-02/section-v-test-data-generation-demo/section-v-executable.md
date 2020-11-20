# Section V - executable

Since the`tdg-service.rs` endpoint is simply a wrapper the routes to the module's functions, we don't need to make any changes in this file.

Restart the web service \(`cargo run`\) and run the `curl-tdg.sh` script multiple times.

```text
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh 
Abrin
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh 
Abry
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh 
Abby
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh 
Abry
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh 
Abby
ArchConfWorkshopUser:~/environment $ ./scripts/curl-tdg.sh 
Abbin
```

{% hint style="warning" %}
Because the `profile`  was generated from a small data sample \(just a few names all starting with `Aa`\), the test data generated is realistically random, but did generate original names by chance. 

The larger the data sample used to create the profile, the more realistic the Test Data and less chance of generating any original data.
{% endhint %}



