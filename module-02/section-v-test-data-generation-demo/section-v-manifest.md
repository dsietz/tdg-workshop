# Section V - mdule

> tdg\_service.toml

To make changes to the logic of our service, we only need to modify the module that the binary service calls.

Let's replace the `Hello Wolrd!` message with some generated test data.

We first make our `use` declarations.

```rust
use test_data_generation::data_sample_parser::DataSampleParser;
```

We then modify our `index` function to load a Data Sample Pasrter file \(`Profile`\) and generate some test data.

