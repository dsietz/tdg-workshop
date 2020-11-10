# Module IV - Building a Genesis Microservice for Processing the Sourced Data

In this module we will create a _Genesis Data Processor_ service. This service is responsible to listen on the `genesis` topic for data that has been sourced, stash this original DaaSDocument in an S3 bucket, and then sending the DaaSDocument down the event flow for provisioning.

