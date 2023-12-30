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

### Comparisons

Here are some comparisons of `runiq` against other methods of filtering uniques, obviously rough as they're run on my machine:

| Tool  | Flags     | Time Taken     | Peak Memory     |
|-------|-----------|----------------|-----------------|
| neek  | N/A       | 55.8s          | 313MB           |
| sort  | -u        | 595s           | 9.07GB          |
| uq    | N/A       | 32.3s          | 1.66GB          |
| runiq | -f digest | **17.8s**      | 64.6MB          |
| runiq | -f naive  | 26.3s          | 1.62GB          |
| runiq | -f bloom  | 36.8s          | **13MB**        |

The numbers above are based on filtering unique values out of the following file:

```
File size:     3,290,971,321 (~3.29GB)
Line count:        5,784,383
Unique count:      2,715,727
Duplicates:        3,068,656
```

In the future a test dataset will be added to the repository to allow for reproducible benchmarks, but at the moment this is on hold due to time constraints (the data used for the initial numbers above is sensitive). If anyone wants to PR something for this(of comparable magnitude), it'd be welcome!
