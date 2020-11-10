# Further Exploration

### Pushing Data Downstream

See if you can change the `order_clothing.rs` processor to forward the DaaSDocument to another topic, \(e.g.: billing\) after it finished its data provisioning.

### Multiple Provisioning of a Single Topic

Try to create a second data provisioning microservice, \(e.g.: order\_clothing\_products that collects a list of unique products ordered\) and listens to the same `order.clothing` topic.

### Advanced: PbD Data Usage Agreement

Try implementing the `PbD` [`Data Usage Agreement`](https://docs.rs/pbd/0.3.0/pbd/dua/index.html) in one of the provisioning microservices so that it only processes data that has an agreement for marketing purposes.

## Helpful Tools

* [Postman Helper](https://github.com/dsietz/daas-sdk/blob/master/examples/postman-helper.rs) - Command line utility to assist in the building of Postman collections for testing your DaaS services

