# statistical-tests-rs
Statistical Tests for Rust

```rs
use statistical_tests_rs::mean;

fn main() {
    // Get the mean of an array
    let array: [f64; 4] = [3.4, 6.7, 2.3, 1.1];
    let m = mean(&array);

    println!("{}", m);
}

```

```rs
use statistical_tests_rs::{GetStatistics, SampleStatistics};

fn main() {
    // Get the statistics of a sample
    let samp = SampleStatistics::from_array(&array);
    println!(
        "Sample Mean: {}, Sample Standard Deviation: {}",
        samp.sample_mean, samp.standard_error
    );
}
```

![image](https://user-images.githubusercontent.com/35516367/169709203-5d375c96-257e-44ff-b400-67f392e820af.png)
