use anyhow::Result;
use libc::exit;
use polars::io::avro::AvroWriter;
use polars::{lazy::dsl::col, prelude::*};
use std::env;
use std::fs::File;
use std::time::Instant;

fn main() -> Result<()> {
    //start timer
    let now = Instant::now();

    // parse args
    let mut args = env::args_os();
    let args_len = args.len();
    let _ = args.next();
    let infile = args.next().unwrap().to_string_lossy().into_owned();
    let outfile = args.last().unwrap().to_string_lossy().into_owned();

    // print usage
    if args_len != 3 {
        println!("Usage: zerodotbrc ../data/100m.csv ./out.avro");
        unsafe {
            exit(1);
        }
    }

    let q = LazyCsvReader::new(infile)
        .with_schema(Some(Arc::new({
            let mut schema = Schema::new();
            schema.with_column("station".to_owned().into(), DataType::String);
            schema.with_column("temp".to_owned().into(), DataType::Float64);
            schema
        })))
        .with_separator(b';')
        .finish()?;

    // This is where most of the work happens. We're grouping by the station,
    // then aggregating on their temps to produce the min, mean, and max temps per station
    let mut df = q
        .with_streaming(true)
        .group_by([col("station")])
        .agg([
            col("temp").min().alias("min").round(1),
            col("temp").mean().alias("mean").round(1),
            col("temp").max().alias("max").round(1),
        ])
        .sort("station", Default::default())
        .collect()?;

    // write out the data frame as Apache Avro
    let mut file = File::create(outfile).expect("could not create file");
    AvroWriter::new(&mut file).finish(&mut df)?;

    // print elapsed time
    let elapsed_time = now.elapsed();
    println!("Took {} seconds.", elapsed_time.as_millis() as f64 / 1000.0);

    Ok(())
}
