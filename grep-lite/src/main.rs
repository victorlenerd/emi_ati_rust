use regex::Regex;
use clap::{App, Arg};

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
        .get_matches();


    let ctx_lines = 2;
    let pattern = args.value_of("pattern").unwrap();
    let haystack = "\
Every face, every shop,
bedroom window, public-house and
dark square is a picture
feverishly turned--in search of what?
It is the same with books.
What do we seek
through millions of pages?";

    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];
    let re = Regex::new(pattern).unwrap();

    for (i, line) in haystack.lines().enumerate() {
        let contains_substring =  re.find(line);

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

    for (i, line) in haystack.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(ctx_lines);
            let upper_bound = tag + ctx_lines;

            if (i >= lower_bound) && (i <= upper_bound) {
                let line_as_string = String::from(line);
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