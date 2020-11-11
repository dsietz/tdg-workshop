# Module V - Building a Provisioning Microservice

In this module we will build a microservice that will

1. consume DaaSDocuments from the `order.clothing` topic
2. sum the number of new clothing orders by product
3. forward the DaaSDocument to the downstream topic

> Notice that the metadata in the DaaSDocument drives the flow and we lay down services for the `data topics` that we wish to provision.

