use clap::Parser;
use rand::{
    distributions::{Distribution, Standard},
    seq::IteratorRandom as _,
    Rng,
};
use std::{
    fs::File,
    io::{BufRead as _, BufReader, Write},
};

enum NextLine {
    Wally,
    Andre,
}

impl NextLine {
    fn other(&self) -> NextLine {
        match self {
            NextLine::Wally => NextLine::Andre,
            NextLine::Andre => NextLine::Wally,
        }
    }
}

impl Distribution<NextLine> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> NextLine {
        // match rng.gen_range(0, 3) { // rand 0.5, 0.6, 0.7
        match rng.gen_range(0..=1) {
            // rand 0.8
            0 => NextLine::Wally,
            1 => NextLine::Andre,
            _ => unreachable!(),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    num: u32,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let reader = BufReader::new(File::open("screenplay.txt")?);
    let mut andres_lines: Vec<String> = Vec::new();
    let mut wallys_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.starts_with("ANDRE:") {
            andres_lines.push(line);
        } else if line.starts_with("WALLY:") {
            wallys_lines.push(line);
        }
    }

    let mut new_screenplay = Vec::new();

    let mut next_line: NextLine = rand::random();
    while !(wallys_lines.is_empty() && andres_lines.is_empty()) {
        match next_line {
            NextLine::Wally => {
                let (i, &ref out) = wallys_lines
                    .iter()
                    .enumerate()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                new_screenplay.push(out.clone());
                wallys_lines.remove(i);
            }
            NextLine::Andre => {
                let (i, &ref out) = andres_lines
                    .iter()
                    .enumerate()
                    .choose(&mut rand::thread_rng())
                    .unwrap();
                new_screenplay.push(out.clone());
                andres_lines.remove(i);
            }
        }

        next_line = next_line.other();
    }

    let mut new_screenplay_file = File::create(format!("new_screenplay{}.txt", args.num))?;
    for line in new_screenplay {
        write!(new_screenplay_file, "{}\n", line)?;
    }

    Ok(())
}
