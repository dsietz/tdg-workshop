# Section IV - feeding the service

Now that the service that will be analyzing the data for `names`, we can start feeding the process.

In the terminal where you started the Kafka producer, start entering some names of people, \(John, Jonny, Jon, Johnathon, Jonathon\)

```text
ArchConfWorkshopUser:~/environment $ kafka_2.13-2.6.0/bin/kafka-console-producer.sh --topic names --bootstrap-server localhost:9092
>John
>Jonny
>Johny
>Jonathon
>Johnathon
>Jon
> 
```

In the terminal where the analyzer is running, you should see output notifying that it created the profile file and analyzed the data.

```text
ArchConfWorkshopUser:~/environment/rust-tdg (master) $ ./target/debug/tdg_analyzer --topic names
Listening to the names topic ...
John
Creating new profile: names
Analyzed the data.
Jonny
Analyzed the data.
Johny
Analyzed the data.
Jonathon
Analyzed the data.
Johnathon
Analyzed the data.
Jon
Analyzed the data.
```

If you open and format the `names.json` file in the `profile` directory, you can see the algorithm that is build from the data that was feed to the `names` topic.

```javascript
{
  "id": "names",
  "patterns": {
    "Cvc": 1,
    "Cvcc": 1,
    "Cvccc": 2,
    "Cvccvccvc": 1,
    "Cvcvccvc": 1
  },
  "pattern_total": 6,
  "pattern_keys": [
    "Cvc",
    "Cvcc",
    "Cvccc",
    "Cvccvccvc",
    "Cvcvccvc"
  ],
  "pattern_vals": [
    1,
    1,
    2,
    1,
    1
  ],
  "pattern_percentages": [
    [
      "Cvccc",
      33.33333333333333
    ],
    [
      "Cvc",
      16.666666666666664
    ],
    [
      "Cvcc",
      16.666666666666664
    ],
    [
      "Cvccvccvc",
      16.666666666666664
    ],
    [
      "Cvcvccvc",
      16.666666666666664
    ]
  ],
  "pattern_ranks": [
    [
      "Cvccc",
      33.33333333333333
    ],
    [
      "Cvc",
      49.99999999999999
    ],
    [
      "Cvcc",
      66.66666666666666
    ],
    [
      "Cvccvccvc",
      83.33333333333331
    ],
    [
      "Cvcvccvc",
      99.99999999999997
    ]
  ],
  "sizes": {
    "3": 1,
    "4": 1,
    "5": 2,
    "8": 1,
    "9": 1
  },
  "size_total": 6,
  "size_ranks": [
    [
      5,
      33.33333333333333
    ],
    [
      3,
      49.99999999999999
    ],
    [
      4,
      66.66666666666666
    ],
    [
      8,
      83.33333333333331
    ],
    [
      9,
      99.99999999999997
    ]
  ],
  "processors": 4,
  "facts": [
    [
      {
        "key": "J",
        "prior_key": null,
        "next_key": "o",
        "pattern_placeholder": "C",
        "starts_with": 1,
        "ends_with": 0,
        "index_offset": 0
      },
      {
        "key": "J",
        "prior_key": null,
        "next_key": "o",
        "pattern_placeholder": "C",
        "starts_with": 1,
        "ends_with": 0,
        "index_offset": 0
      },
      {
        "key": "y",
        "prior_key": "n",
        "next_key": null,
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 1,
        "index_offset": 4
      },
      {
        "key": "J",
        "prior_key": null,
        "next_key": "o",
        "pattern_placeholder": "C",
        "starts_with": 1,
        "ends_with": 0,
        "index_offset": 0
      },
      {
        "key": "y",
        "prior_key": "n",
        "next_key": null,
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 1,
        "index_offset": 4
      },
      {
        "key": "J",
        "prior_key": null,
        "next_key": "o",
        "pattern_placeholder": "C",
        "starts_with": 1,
        "ends_with": 0,
        "index_offset": 0
      },
      {
        "key": "t",
        "prior_key": "a",
        "next_key": "h",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 4
      },
      {
        "key": "J",
        "prior_key": null,
        "next_key": "o",
        "pattern_placeholder": "C",
        "starts_with": 1,
        "ends_with": 0,
        "index_offset": 0
      },
      {
        "key": "a",
        "prior_key": "n",
        "next_key": "t",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 4
      },
      {
        "key": "n",
        "prior_key": "o",
        "next_key": null,
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 1,
        "index_offset": 8
      },
      {
        "key": "J",
        "prior_key": null,
        "next_key": "o",
        "pattern_placeholder": "C",
        "starts_with": 1,
        "ends_with": 0,
        "index_offset": 0
      }
    ],
    [
      {
        "key": "o",
        "prior_key": "J",
        "next_key": "h",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 1
      },
      {
        "key": "o",
        "prior_key": "J",
        "next_key": "n",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 1
      },
      {
        "key": "o",
        "prior_key": "J",
        "next_key": "h",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 1
      },
      {
        "key": "o",
        "prior_key": "J",
        "next_key": "n",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 1
      },
      {
        "key": "h",
        "prior_key": "t",
        "next_key": "o",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 5
      },
      {
        "key": "o",
        "prior_key": "J",
        "next_key": "h",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 1
      },
      {
        "key": "t",
        "prior_key": "a",
        "next_key": "h",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 5
      },
      {
        "key": "o",
        "prior_key": "J",
        "next_key": "n",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 1
      }
    ],
    [
      {
        "key": "h",
        "prior_key": "o",
        "next_key": "n",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 2
      },
      {
        "key": "n",
        "prior_key": "o",
        "next_key": "n",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 2
      },
      {
        "key": "h",
        "prior_key": "o",
        "next_key": "n",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 2
      },
      {
        "key": "n",
        "prior_key": "o",
        "next_key": "a",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 2
      },
      {
        "key": "o",
        "prior_key": "h",
        "next_key": "n",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 6
      },
      {
        "key": "h",
        "prior_key": "o",
        "next_key": "n",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 2
      },
      {
        "key": "h",
        "prior_key": "t",
        "next_key": "o",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 6
      },
      {
        "key": "n",
        "prior_key": "o",
        "next_key": null,
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 1,
        "index_offset": 2
      }
    ],
    [
      {
        "key": "n",
        "prior_key": "h",
        "next_key": null,
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 1,
        "index_offset": 3
      },
      {
        "key": "n",
        "prior_key": "n",
        "next_key": "y",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 3
      },
      {
        "key": "n",
        "prior_key": "h",
        "next_key": "y",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 3
      },
      {
        "key": "a",
        "prior_key": "n",
        "next_key": "t",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 3
      },
      {
        "key": "n",
        "prior_key": "o",
        "next_key": null,
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 1,
        "index_offset": 7
      },
      {
        "key": "n",
        "prior_key": "h",
        "next_key": "a",
        "pattern_placeholder": "c",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 3
      },
      {
        "key": "o",
        "prior_key": "h",
        "next_key": "n",
        "pattern_placeholder": "v",
        "starts_with": 0,
        "ends_with": 0,
        "index_offset": 7
      }
    ]
  ]
}
```

