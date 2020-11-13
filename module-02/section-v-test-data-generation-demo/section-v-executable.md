# Section V - executable

Since the`tdg-service.rs` endpoint i simply a wrapper the routes to the module's functions, we don't need to make any changes in this file.

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

Notice that the name generated is based on the 

