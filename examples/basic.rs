use jen::generator::Generator;
use runiq::{Filter, QuickFilter};

fn main() {
    // First we create a filter to detect duplicates
    let mut filter = QuickFilter::default();

    // As well as a template to generate random inputs
    let template = "{{ integer(start=1, end=6) }}";

    // Then we generate some random input values
    let inputs = Generator::from_string(template)
        .unwrap()
        .take(20)
        .collect::<Vec<_>>();

    // And filter uniques
    let outputs = inputs
        .iter()
        .filter(|v| filter.detect(v.as_bytes()))
        .map(|v| v.to_owned())
        .collect::<Vec<_>>();

    // Before we print the before/after to the console
    println!("Generate values: {}", inputs.join(", "));
    println!("Filtered values: {}", outputs.join(", "));
}
