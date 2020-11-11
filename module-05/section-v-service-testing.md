# Section V - service testing

Since the genesis service is not a RESTful service, we will have to verify that it is successfully running using Kafka consumer and a S3 Bucket lookup.

## Kafka

The genesis service dynamically creates topics based on the metadata of the DaaSDocument that is consumes.

Let's see what topics it has created.

> NOTE: You will want to run this in a new terminal.

```text
./kafka_2.13-2.6.0/bin/kafka-topics.sh --list --zookeeper localhost:2181
```

You should see the following topics in the results.

```text
__consumer_offsets
genesis
iStore
order
order.clothing
order.clothing.iStore
```

The genesis service parsed the parameters in the resource path of the RESTful call and created topics accordingly.

`http://localhost:8000/order/clothing/iStore/5000`

This feature allows us to create downstream provisioning services based on the data we wish to process. \(We'll see this in the next module.\)

Let's look at the documents that have been sent to one of these topics.

```text
./kafka_2.13-2.6.0/bin/kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic order.clothing --from-beginning
```

## S3 Bucket

Before the genesis service send copies of the DaaSDocument downstream to be provisioned, it first stored the original copy in the S3 bucket that was configured in the `gensis.rs` file.

```rust
// NOTE: Modify the Bucket name to match your bucket
// Credentials are read from the environment variables AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY
pub const BUCKET_NAME: &'static str = "iapp-archconf-workshop";
```

To confirm that the DaaSDocument has been stored in the S3 bucket, run the following command.

```text
aws s3api list-objects --bucket iapp-archconf-workshop --prefix genesis --query 'Contents[].{Key: Key, Size: Size}'
```

A json array should be returned with the file\(s\) `.daas`

```text
ArchConfWorkshopUser:~/environment $ aws s3api list-objects --bucket iapp-archconf-workshop --prefix genesis --query 'Contents[].{Key: Key, Size: Size}'                                                                                                                      
[
    {
        "Key": "genesis/order~clothing~iStore~5000.daas", 
        "Size": 706
    }
]
```

Try to look at the versions

```text
aws s3api list-object-versions --bucket iapp-archconf-workshop --prefix genesis
```

```javascript
{
    "Versions": [
        {
            "LastModified": "2020-11-04T22:03:06.000Z", 
            "VersionId": "PMQQBJ6T2ve8qAD45J.ELqMSpNJX6.fW", 
            "ETag": "\"295472db3011b179b342b010fcee20e3\"", 
            "StorageClass": "STANDARD", 
            "Key": "genesis/order~clothing~iStore~5000.daas", 
            "IsLatest": true, 
            "Size": 706
        }, 
        {
            "LastModified": "2020-11-04T21:28:29.000Z", 
            "VersionId": "gbNzGC825DGEgAcfNJgxDOFt702q.jlZ", 
            "ETag": "\"efca02daba2596ea31ed02544eed7f3e\"", 
            "StorageClass": "STANDARD", 
            "Key": "genesis/order~clothing~iStore~5000.daas", 
            "IsLatest": false, 
            "Size": 706
        }, 
        {
            "LastModified": "2020-11-04T21:28:29.000Z", 
            "VersionId": "0jR5YYDmN4REFMskajl.NrerSR3E0iUr", 
            "ETag": "\"714eddb4c1424840d96d319f537cf9a3\"", 
            "StorageClass": "STANDARD", 
            "Key": "genesis/order~clothing~iStore~5000.daas", 
            "IsLatest": false, 
            "Size": 706
        }
    ]
}
```

