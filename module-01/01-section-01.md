# Section I - Overview

The availability of Test Data remains to be a difficult challenge for any testing framework - manual or automated. In order to be effective test data, it must be:

* Realistic
* Timely
* Subsets
* Repeatable

### Realistic

In order for the Test Data to be realistic, it must effectively represent the **variety**, **size** and **complexity** of the original data. It must be a accurate profile of the data you wish to mimic.

### Timely

Untimely Test Data can directly impact our ability to test our solutions and hinder our devlier time. Waiting for Test Data to be _reset_ after running tests, \(e.g.: scheduled batch jobs\) is a common point of frustration.

### Subsets

Too often, refreshing the Test Data is an cumbersome process. Because of data modeling constraints, \(e.g.: relational constraints\) and poorly automated designs, \(e.g.: database schema or table restores\) development teams find themselves _parasitically coupled_ by the data sources they share - even if they don't use the same data records. 

### Repeatable

We all know that tests fail - otherwise there wouldn't be a need for testing. Resetting Test Data back to its desired state is a basic property of any testing platform. One _sniff_ that can be used to identify this inability, is the continuous growth of data in the testing data source, \(e.g.: database size grows at a 15% rate per day\). This is because tester create new test data instead of reusing the data.   

