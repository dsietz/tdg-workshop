# Section V - module

> tdg\_service.toml

To make changes to the logic of our service, we only need to modify the module that the binary service calls.

Let's replace the `Hello Wolrd!` message with some generated test data.

We first make our `use` declarations.

```rust
use test_data_generation::data_sample_parser::DataSampleParser;
```

Next, add a global constant under the `use` declarations that will be referenced in our funciton.

```rust
static WORKSPACE_LOCAL_STORAGE: &str = "../scripts";
```

We then modify our `index` function to load a Data Sample Parser file \(`Profile`\) and generate some test data.

```rust
pub fn index(_req: HttpRequest) -> HttpResponse {
    let dsp_file = &format!("{}/{}", WORKSPACE_LOCAL_STORAGE, "sample-01-dsp");
    let mut dsp = DataSampleParser::from_file(&dsp_file);

    HttpResponse::build(StatusCode::OK)
    .body(dsp.generate_record()[0].to_string())
}
```

