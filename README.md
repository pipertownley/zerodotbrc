# 0.1BRC Challenge Polars
> A slightly modified version of 1BRC, and 1/10th the size

As my first small project with Polars-rs, I decided to take a swing at the 1 Billion Row Challenge.

The 1BRC, created by Gunnar Morling, was originally a Java performance competition to process a 1 billion row CSV file. [\[1\]](https://github.com/gunnarmorling/1brc)

The test bed for the competition was a Hetzner AX161 dedicated server with 32 code AMD Ryzen 7 and 128GB of ram.

However, as I'm running this on a MacBook M1 Pro with 10 cores and 16GB of ram, I've scaled down to a 100M row version.

I've modified the original requirements to output to STDOUT and instead write to an Apache Avro file.

<div>
    <img src="assets/run.png" width=60%/>
</div>

not too shabby, eh?

## Installation

Download the latest release to your path.

## Usage example

`./zerodotbrc [infile] [outfile]`

## Development setup

```sh
git clone https://github.com/pipertownley/zerodotbrc
cd zerodotbrc
```

```sh
cargo build --release
cp target/release/zerodotbrc /usr/local/bin
```

## Release History

* 0.1.0
    * The first release
* 0.1.1
    * Fix panic from missing args
