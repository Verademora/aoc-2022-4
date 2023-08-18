use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
     // Open our input file and read it to a String
    let mut file = File::open("src/input")?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    // Split the input and collect
    let v: Vec<&str> = buffer.trim().split('\n').collect();
    let pairs: Vec<Vec<&str>> = v
        .iter()
        .map(|s| s.split(',').collect::<Vec<&str>>())
        .collect();
        for pair in pairs {
            if let [first, second] = pair[..] {
                let v = make_array(first)?;
                dbg!(v);
            }
        }

    Ok(())
}

fn make_array(array_str: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let v: Vec<&str> = array_str.split('-').collect();
    if let [start, end] = v[..] {
        // dbg!(start);
        // dbg!(end);
        // let startn = start.parse()?;
        // let endn = start.parse()?;
        let result = vec![start, end];
    };
    
    Ok(result)
}
