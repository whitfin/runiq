# runiq
[![Build Status](https://img.shields.io/github/actions/workflow/status/whitfin/runiq/rust.yml?branch=main)](https://github.com/whitfin/runiq/actions) [![Crates.io](https://img.shields.io/crates/v/runiq.svg)](https://crates.io/crates/runiq)

This project offers an efficient way (in both time and space) to filter duplicate entries (lines) from texual input. This project was born from [neek](https://github.com/whitfin/neek), but optimized for both speed and memory. Several filtering options are supported depending on your data and tradeoffs you wish to make between speed and memory usage. For a more detailed explanation, see the relevant [blog post](https://whitfin.io/filtering-unique-logs-using-rust/).

### Installation

Runiq will be available via [Crates.io](https://crates.io/crates/runiq), so it can be installed from there directly. You can use Runiq either as a command line utility, or directly via the programmatic API.

If you wish to install Runiq as a command line utility, you can install it via an easy one-liner in your terminal:

```shell
$ cargo install runiq
```

If you wish to use it as a library, you can add it to your `Cargo.toml` as a dependency of your application:

```toml
[dependencies]
runiq = { version = "2.0", default-features = false }
```

You should disable the default features as it includes several dependencies which are required for the CLI use case. These dependencies are not included in your application when these features are disabled.

### Examples

Below are a few examples of using the Runiq CLI to filter duplicates out of input text.

```shell
$ cat << EOF >> input.txt
this is a unique line
this is a duplicate line
this is another unique line
this is a duplicate line
this is a duplicate line
EOF

$ cat input.txt
this is a unique line
this is a duplicate line
this is another unique line
this is a duplicate line
this is a duplicate line

$ runiq input.txt
this is a unique line
this is a duplicate line
this is another unique line
```

For examples of the programmatic API, please see [the examples](./examples/basic.rs).

### Filters

Runiq comes with several "filters", which control exactly how uniqueness is verified. Each of these filters has different use cases, and excels in different ways.

* `quick`
    * The `quick` filter works the same way as the `simple` filter, except values are pre-hashed.
    * This results in much lower memory overhead than `simple`, with comparably throughput.
    * Depending on your input lengths, throughput can actually be faster than `simple`.
* `simple`
    * The `simple` filter uses basic `Set` implementations to determine uniqueness.
    * Offers a fairly good throughput, while still effectively guaranteeing accuracy.
    * As all inputs are stored, the memory requirement scales linearly to your input sizes.
* `sorted`
    * The `sorted` filter acts much the same way as the standard `uniq` tool, by only detecting sequential duplicates.
    * This is naturally extremely low on resources, with very minimal memory overhead.
    * Obviously has a requirement that your input values be sorted.
* `compact`
    * The `compact` filter (heh) uses a scaling Bloom Filter to determine uniqueness.
    * Performs very quickly due to small structures, with a minimal memory overhead.
    * Perfect accuracy is no longer guaranteed; there can be rare cases of false positives.
    * Best used for statistics on files, although will remain near perfect for millions of records.
    * See the comparisons below for some notes on accuracy of this filter.

### Comparisons

To grab some rough comparisons of `runiq` against other methods of filtering uniques, we can use some sample data. This data is generated via [Jen](https://github.com/whitfin/jen) using the templates provided in the corresponding directory. You can create your own templates to more closely match your use case for a better comparison.

To start with, we'll generate a sample dataset of 25,000,000 JSON documents using the [basic](./templates/basic.tera) template. This template will result in an approximate 20% duplication rate (randomly dotted around the file) at this scale. Note that for longer inputs, you can tweak the `rp` value inside the template to cause repetition of fields.

```
$ jen templates/basic.tera -l 25000000 > 25000000.jsonl

File Size:     1,913,658,811 (~1.9 GB)
Total Count:      25,000,000
Unique Count:     19,832,571
Dup Offset:        5,167,429
Dup Rate:             20.67%
```

We can then run this sample dataset through the various filters of `runiq`, as well as some other tools to gauge how we're doing. These numbers are *not* meant to be a competition. They are simply a point of reference for myself when testing improvements. It is definitely possible that other tools might fit your data shape better.

| Tool  | Flags      | Time (Unsorted) | Memory (Unsorted) | Time (Sorted) | Memory (Sorted) |
|:------|:-----------|----------------:|------------------:|--------------:|----------------:|
| uniq  | N/A        | N/A             | N/A               | 24.9s         | 1.6MB           |
| sort  | -u         | 380.2s          | 8.33GB            | 58.7s         | 8.15GB          |
| uq    | N/A        | 22.6s           | 2.34GB            | 21.0s         | 2.34GB          |
| huniq | N/A        | 11.9s           | 298.5MB           | 11.6s         | 300.7MB         |
| runiq | -f quick   | 12.1s           | 298.7MB           | 11.8s         | 298.5MB         |
| runiq | -f simple  | 19.7s           | 2.33GB            | 18.2s         | 2.33GB          |
| runiq | -f sorted  | N/A             | N/A               | 10.3s         | 1.3MB           |
| runiq | -f compact | 17.8s           | 162.2MB           | 16.2s         | 162.3MB         |

For another point of comparison, we'll repeat these tests with a sample of 100,000,000 JSON documents (so 4x the first test). In this case, the duplicate rate will rise to approximtely 55% using the same template:

```
$ jen templates/basic.tera -l 100000000 > 100000000.jsonl

File Size:     7,654,658,706 (~7.7 GB)
Total Count:     100,000,000
Unique Count:     44,305,712
Dup Offset:       55,694,288
Dup Rate:             55.69%
```

| Tool  | Flags      | Time (Unsorted) | Memory (Unsorted) | Time (Sorted) | Memory (Sorted) |
|:------|:-----------|----------------:|------------------:|--------------:|----------------:|
| uniq  | N/A        | N/A             | N/A               | 105.8s        | 1.6MB           |
| sort  | -u         | 2529.9s         | 12.70GB           | 373.0s        | 12.42GB         |
| uq    | N/A        | 76.4s           | 5.03GB            | 57.9s         | 5.03GB          |
| huniq | N/A        | 31.2s           | 586.3MB           | 28.4s         | 587.4MB         |
| runiq | -f quick   | 34.7s           | 586.8MB           | 30.5s         | 586.6MB         |
| runiq | -f simple  | 67.4s           | 5.00GB            | 49.0s         | 5.00GB          |
| runiq | -f sorted  | N/A             | N/A               | 24.9s         | 1.3MB           |
| runiq | -f compact | 66.3s           | 338.3MB           | 49.0s         | 338.3M          |

All of these numbers are with the tool output being written to `/dev/null`. Some of these tools (`runiq` included) have flags to count/report rather than print the outputs; these use cases will always be much quicker than the numbers above.

It's also worth noting the accuracy given by the `compact` filter in these cases above; in both of my test sets the results were identical to those of the other filter types, showing that the `compact` filter is generally pretty acurrate to some fairly large amounts of input (although not always!).
