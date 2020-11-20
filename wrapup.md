# Further Exploration

### Add Another Data Topic

What you would have to do to add a new data topic to the test data generation platform for generating `birth dates`?

### Analyzing Data Records

Implement a new data topic that analyzes data records \(CSV formed data\). This can be done using the [`DataSampleParser::analyze_csv_data()` ](https://docs.rs/test-data-generation/0.2.1/test_data_generation/data_sample_parser/struct.DataSampleParser.html#method.analyze_csv_data)function. 

### Improving Realisticity

The Test Data Generation library comes with a helpful feature to improve the realisticity of the generated data to be 80% realistic using the [`Levenshtein Distance`](https://en.wikipedia.org/wiki/Levenshtein_distance) . This can be done by calling the [`Profile::learn_from_entity()`](https://docs.rs/test-data-generation/0.2.1/test_data_generation/struct.Profile.html#method.learn_from_entity) function.

{% hint style="info" %}
This should be a separate RESTful endpoint that refactors the specified saved Profile based on the data that was provided for analysis.
{% endhint %}



