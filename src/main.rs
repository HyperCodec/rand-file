use std::path::PathBuf;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::{fs::OpenOptions, io::AsyncWriteExt};
use rayon::prelude::*;

#[derive(Parser)]
#[command(
    name = "rand-file",
    author = "HyperCodec",
    about = "Writes a big file with random bytes"
)]
struct Cli {
    #[arg(short, long, help = "The amount of bytes per file append", default_value = "1000000")]
    buf_size: usize,

    #[arg(short, long, help = "The total amount of data to write", default_value = "10000000000")]
    total_data: u64,

    #[arg(help = "The path to write the big file")]
    path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(args.path)
        .await?;

    let pb = ProgressBar::new(args.total_data);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut total_written = 0;
    // let mut iter_count = 0;
    while total_written < args.total_data {
        // allocating and then storing is quite a bit faster i think
        let mut buf = vec![0; args.buf_size];

        buf
            .par_iter_mut()
            .for_each(|b| *b = fastrand::u8(0..u8::MAX));
        // let buf: Vec<_> = (0..args.buf_size)
        //     .into_par_iter()
        //     .map(|_| fastrand::u8(0..u8::MAX))
        //     .collect();
        
        let written = file.write(&buf).await?;
        total_written += written as u64;
        // iter_count += 1;
        pb.set_position(total_written);
    }
    
    // println!("avg successful bytes written: {}", total_written as f32 / iter_count as f32);

    pb.finish();

    Ok(())
}
