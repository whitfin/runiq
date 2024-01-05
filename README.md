# runiq
[![Build Status](https://img.shields.io/github/actions/workflow/status/whitfin/runiq/rust.yml?branch=main)](https://github.com/whitfin/runiq/actions) [![Crates.io](https://img.shields.io/crates/v/runiq.svg)](https://crates.io/crates/runiq)

This project offers an efficient way (in both time and space) to filter duplicate entries (lines) from texual input. This project was born from [neek](https://github.com/whitfin/neek), but optimized for both speed and memory. Several filtering options are supported depending on your data and tradeoffs you wish to make between speed and memory usage. For a more detailed explanation, see the relevant [blog post](https://whitfin.io/filtering-unique-logs-using-rust/).

### Installation

This tool will be available via [Crates.io](https://crates.io/crates/runiq), so you can install it directly with `cargo`:

```shell
$ cargo install runiq
```

If you'd rather just grab a pre-built binary, you might be able to download the correct binary for your architecture directly from the latest release on GitHub [here](https://github.com/whitfin/runiq/releases). The list of binaries may not be complete, so please file an issue if your setup is missing (bonus points if you attach the appropriate binary).

### Examples

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

### Filters

Runiq comes with several "filters", which control exactly how uniqueness is verified. Each of these filters has different use cases, and excels in different ways.

* `sorted`
    * The `sorted` filter acts much the same way as the standard `uniq` tool, by only detecting sequential duplicates.
    * This is naturally extremely low on resources, with very minimal memory overhead.
    * Obviously has a requirement that your input values be sorted.
* `naive`
    * The `naive` filter uses basic `Set` implementations to determine uniqueness.
    * Offers a fairly good throughput, while still effectively guaranteeing accuracy.
    * As all inputs are stored, the memory requirement scales linearly to your input sizes.
* `digest`
    * The `digest` filter works the same way as the `naive` filter, except values are pre-hashed.
    * This results in much lower memory overhead than `naive`, with comparably throughput.
    * Depending on your input lengths, throughput can actually be faster than `naive`.
* `bloom`
    * The `bloom` filter (heh) uses a scaling Bloom Filter to determine uniqueness.
    * Performs very quickly due to small structures, with a minimal memory overhead.
    * Perfect accuracy is no longer guaranteed; there can be rare cases of false positives.
    * Best used for statistics on files, although will remain near perfect for millions of records.
    * See the comparisons below for some notes on accuracy of this filter.

### Comparisons

To grab some rough comparisons of `runiq` against other methods of filtering uniques, we can use some sample data. This data is generated via [Jen](https://github.com/whitfin/jen) using the templates provided in the corresponding directory. You can create your own templates to more closely match your use case for a better comparison.

To start with, we'll generate a sample dataset of 25,000,000 JSON documents using the [basic](./templates/basic.tera) template. This template will result in an approximate 20% duplication rate (randomly dotted around the file) at this scale.

```
$ jen templates/basic.tera -l 25000000 > 25000000.jsonl

File Size:     1,813,645,568 (~1.8 GB)
Total Count:      25,000,000
Unique Count:     19,833,427
Dup Offset:        5,166,573
Dup Rate:             20.67%
```

We can then run this sample dataset through the various filters of `runiq`, as well as some other tools to gauge how we're doing. These numbers are *not* meant to be a competition. They are simply a point of reference for myself when testing improvements. It is definitely possible that other tools might fit your data shape better.

| Tool  | Flags     | Time (Unsorted) | Memory (Unsorted) | Time (Sorted) | Memory (Sorted) |
|:------|:----------|----------------:|------------------:|--------------:|----------------:|
| uniq  | N/A       | N/A             | N/A               | 26.3s         | 1.6MB           |
| sort  | -u        | 369.7s          | 7.95GB            | 71.6s         | 7.77GB          |
| uq    | N/A       | 22.7s           | 2.31GB            | 22.7s         | 2.31GB          |
| huniq | N/A       | 11.9s           | 298.5MB           | 11.6s         | 299.8MB         |
| runiq | -f naive  | 19.8s           | 2.28GB            | 18.7s         | 2.29GB          |
| runiq | -f digest | 12.2s           | 298.7MB           | 11.8s         | 299.6MB         |
| runiq | -f bloom  | 17.8s           | 162.4MB           | 17.1s         | 162.3MB         |
| runiq | -f sorted | N/A             | N/A               | 10.4s         | 1.3MB           |

For another point of comparison, we'll repeat these tests with a sample of 100,000,000 JSON documents (so 4x the first test). In this case, the duplicate rate will rise to approximtely 55% using the same template:

```
$ jen templates/basic.tera -l 100000000 > 100000000.jsonl

File Size:     7,254,585,942 (~7.3 GB)
Total Count:     100,000,000
Unique Count:     44,302,820
Dup Offset:       55,697,180
Dup Rate:             55.70%
```

| Tool  | Flags     | Time (Unsorted) | Memory (Unsorted) | Time (Sorted) | Memory (Sorted) |
|:------|:----------|----------------:|------------------:|--------------:|----------------:|
| uniq  | N/A       | N/A             | N/A               | 98.7s         | 1.8MB           |
| sort  | -u        | 2717.8s         | 12.72GB           | 358.7s        | 12.42GB         |
| uq    | N/A       | 76.7s           | 4.93GB            | 58.8s         | 4.93GB          |
| huniq | N/A       | 31.5s           | 587.3MB           | 29.8s         | 589.2MB         |
| runiq | -f naive  | 67.3s           | 4.91GB            | 49.7s         | 4.91GB          |
| runiq | -f digest | 34.8s           | 586.8MB           | 31.1s         | 588.6MB         |
| runiq | -f bloom  | 66.5s           | 338MB             | 49.8s         | 338MB           |
| runiq | -f sorted | N/A             | N/A               | 24.9s         | 1.3MB           |

All of these numbers are with the tool output being written to `/dev/null`. Some of these tools (`runiq` included) have flags to count/report rather than print the outputs; these use cases will always be much quicker than the numbers above.

It's also worth noting the accuracy given by the `bloom` filter in these cases above. In the first test the results were identical to those of the other filter types, showing that it's generally pretty accurate to some fairly large amounts of input (although not always!). In the case of the second test set, it actually only reported a single false positive (it reported 44,302,819 uniques). This should show some scale of accuracy; it will generally be at least _near_ perfect, with lower amounts of input data basically always being correct.
