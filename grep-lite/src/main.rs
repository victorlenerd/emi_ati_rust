use regex::Regex;
use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    let ctx_lines = 2;
    let ref lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    let ilines = lines.into_iter();    for (i, line) in ilines.enumerate() {
        let _line = &line;
        let contains_substring =  re.find(_line);

        match contains_substring {
            Some(_) => {
                tags.push(i);

                let v = Vec::with_capacity(2*ctx_lines + 1);
                ctx.push(v);
            },
            None => (),
        }
    }

    if tags.is_empty() {
        return;
    }

    let ilines = lines.into_iter();
    for (i, line) in ilines.enumerate() {
        let _line = line;
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(_line);
                let local_ctx = (i ,line_as_string);
                ctx[j].push(local_ctx)
            }
        }
    }

    for local_ctx in ctx.iter() {
        println!("-------");
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i + 1;
            println!("{}: {}", line_num, line);
        }
    }
}

fn main() {
    let args = App::new("grep-lite")
        .version("1.0")
        .about("searches for patterns")
        .arg(
            Arg::with_name("pattern")
                .help("The pattern tp search for")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("input")
            .help("File to search")
            .takes_value(true)
            .required(true)
        )
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let input = args.value_of("input").unwrap();
    let re = Regex::new(pattern).unwrap();

    if input == "-" {
        let stdin = io::stdin();
        let reader = stdin.lock();
        process_lines(reader, re)
    } else {
        let f = File::open(input).unwrap();
        let reader = BufReader::new(f);
        process_lines(reader, re);
    }

}