# Section II - Current State Design

Most testing frameworks have not moved beyond the traditional monolithic pattern and data batching.

![](../.gitbook/assets/tdg-01.jpg)

### Effectiveness

Traditional testing frameworks are far from timely. The process for adding new test data from an original source, \(e.g.: production data\) is complex and a lengthy process. Tight coupling between the original data source, \(e.g.: relational data model\) and test definitions, \(e.g.: scheme-less documents\) becomes an ever lasting excercise in data mapping complexity.

### Security

In order to be compliant with security and data privacy rules, the extraction, cleansing, and landing of production data as it feeds the test data becomes a point of contention. 

### Privacy by Design

