fn main() {
    let penguin_data = "\
    common name, length (cm)
    little p, 22
    big pg, 22
    ";

    let records = penguin_data.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue
        }

        let fields: Vec<_> = record
            .split(",")
            .map(|field| field.trim())
            .collect();

        if cfg!(debug_assertions) {
            eprint!("debug: {:?} -> {:?}", record, fields);
        }

        let name = fields[0];
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, length)
        }
    }
}