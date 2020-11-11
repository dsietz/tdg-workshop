# Section VI - testing the service

## Step 1 - Checking the services

Let's first make sure all our services are running and restart our reporting service.

In a new terminal, run the following command:

```text
./rust-daas/target/debug/myapp_reporting
```

In another terminal, let's run the sourcing script.

```text
./scripts/curl-sourcing.sh
```

You should see all the services printing to the console about the data they have touched.

### Sourcing RESTful service

```text
[2020-11-09T13:24:06Z INFO  actix_web::middleware::logger] 127.0.0.1:33482 curl/7.61.1
[2020-11-09T13:24:06Z INFO  actix_web::middleware::logger] 127.0.0.1:33482 "POST /order/clothing/iStore/5000 HTTP/1.1" 200 15 "-" "curl/7.61.1" 0.002745
```

### Genesis service

```text
[2020-11-09T13:24:06Z INFO  daas::service::processor] Putting document order~clothing~iStore~5000 in S3
[2020-11-09T13:24:06Z INFO  daas::service::processor] Brokering document order~clothing~iStore~5000 ...
```

### Order Clothing service

```text
ArchConfWorkshopUser:~/environment $ ./rust-daas/target/debug/myapp_order_clothing 
Clothing Orders processor is running ...
Press [Enter] to stop the Clothing Orders processor.
Order Number 5000 from the iStore has a status of "new"...
Retreiving leather_jacket file
```

### Reporting service

```text
ArchConfWorkshopUser:~/environment $ ./rust-daas/target/debug/myapp_reporting
```

## Step 2 - Calling the Reporting RESTful service

Let's first make sure the returned payload is correct based on the resource path.

In an available terminal, run the following script using a specific product

```text
./scripts/curl-reporting.sh -p "leather jacket"
```

> NOTE: The JSON is an object for the specific product.

```javascript
{"orders":6}
```

Now let's make sure the payload is correct when a product is not specified.

```javascript
./scripts/curl-reporting.sh
```

> NOTE: The JSON is an array verses an object

```javascript
[{"orders":6,"product":"leather jacket"}]
```

### Step 3 - Testing the Data Provisioning Flow

We still haven't verified that our DaaS platform is working when sourcing dynamic content. Let's being by sourcing some variable content.

Open the `curl-sourcing.sh` script in the `./scrips` directory so that we are ordering a different product.

> IMPORTANT: Don't forget to save the file

```bash
curl --location --request POST 'http://localhost:8000/order/clothing/iStore/5000' \
--header 'Data-Usage-Agreement: [{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]' \
--header 'Content-Type: application/json' \
--header 'Data-Tracker-Chain: W3siaWRlbnRpZmllciI6eyJkYXRhX2lkIjoib3JkZXJ+Y2xvdGhpbmd+aVN0b3JlfjUwMDAiLCJpbmRleCI6MCwidGltZXN0YW1wIjowLCJhY3Rvcl9pZCI6IiIsInByZXZpb3VzX2hhc2giOiIwIn0sImhhc2giOiI3MjI1OTUwMzMyNzI3NjAyMDk1MjEwMjM2ODY3MjE0ODM1ODQ4NSIsIm5vbmNlIjo1fV0=' \
--header 'Authorization: Basic aXN0b3JlX2FwcDpzZWNyZXQ=' \
--data-raw '{
    "product":"wool hat",
    "quantity": 1,
    "status":"new"
}'
```

Rerun the `./scripts/curl-sourcing.sh` command.

We want to make sure the order has been properly aggregated to our reporting data source, so let's rerun the `./scripts/curl-reporting.sh` command

```javascript
[{"orders":6,"product":"leather jacket"},{"orders":1,"product":"wool hat"}]
```

Let's make another change to the `curl-sourcing.sh` script by changing the name of the store from `iStore` to `myStore`

```javascript
curl --location --request POST 'http://localhost:8000/order/clothing/myStore/5000' \
--header 'Data-Usage-Agreement: [{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1553988607}]' \
--header 'Content-Type: application/json' \
--header 'Data-Tracker-Chain: W3siaWRlbnRpZmllciI6eyJkYXRhX2lkIjoib3JkZXJ+Y2xvdGhpbmd+aVN0b3JlfjUwMDAiLCJpbmRleCI6MCwidGltZXN0YW1wIjowLCJhY3Rvcl9pZCI6IiIsInByZXZpb3VzX2hhc2giOiIwIn0sImhhc2giOiI3MjI1OTUwMzMyNzI3NjAyMDk1MjEwMjM2ODY3MjE0ODM1ODQ4NSIsIm5vbmNlIjo1fV0=' \
--header 'Authorization: Basic aXN0b3JlX2FwcDpzZWNyZXQ=' \
--data-raw '{
    "product":"wool hat",
    "quantity": 1,
    "status":"new"
}'
```

After you rerun the `./scripts/curl-sourcing.sh` script, you should get a payload returned stating that the data cannot be processed.

```javascript
{"error":"unable to process data"}
```

This is because the DaaS SDK automatically verifies that the data being sent is coming from the original source that created it. This is possible because of the `Data Tracker Chain` feature from the `pbd`.

Update the `curl-sourcing.sh` script to the following, which has a `Data-Tracker-Chain` value that matches the resource path `order/clothing/myStore/5000`:

```rust
curl --location --request POST 'http://localhost:8000/order/clothing/myStore/5000' \
--header 'Data-Usage-Agreement: [{"agreement_name":"billing","location":"www.dua.org/billing.pdf","agreed_dtm": 1582559823}]' \
--header 'Content-Type: application/json' \
--header 'Data-Tracker-Chain: W3siaWRlbnRpZmllciI6eyJkYXRhX2lkIjoib3JkZXJ+Y2xvdGhpbmd+bXlTdG9yZX41MDAwIiwiaW5kZXgiOjAsInRpbWVzdGFtcCI6MCwiYWN0b3JfaWQiOiIiLCJwcmV2aW91c19oYXNoIjoiMCJ9LCJoYXNoIjoiMTMzOTkzNzg5NjgyOTI0MTk5NzM2NDIzOTE5MDUwNDU1NjA2Mjc0Iiwibm9uY2UiOjV9XQ==' \
--header 'Authorization: Basic aXN0b3JlX2FwcDpzZWNyZXQ=' \
--data-raw '{
    "product":"wool hat",
    "quantity": 1,
    "status":"new"
}'
```

You should be able to confirm the following items:

### Sourcing

* `{"status":"ok"}` response
* `./local_storage/clothing/myStore/5000` directory with a your DaaSDocument json file

### Genesis Processor

```rust
[2020-11-09T19:52:44Z INFO  daas::service::processor] Putting document order~clothing~myStore~5000 in S3
[2020-11-09T19:52:44Z INFO  daas::service::processor] Brokering document order~clothing~myStore~5000 ...
```

Kafka topics dynamically created

```javascript
./kafka_2.13-2.6.0/bin/kafka-topics.sh --list --bootstrap-server localhost:9092 
__consumer_offsets
genesis
iStore
myStore
order
order.clothing
order.clothing.iStore
order.clothing.myStore
```

### Provisioning Processor

```rust
Order Number 5000 from the myStore has a status of "new"...
Retreiving wool_hat file
```

### Reporting

```javascript
[{"orders":6,"product":"leather jacket"},{"orders":2,"product":"wool hat"}]
```

