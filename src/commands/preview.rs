use super::metafiles::Snapshot;
use crate::Error;
use std::fs;
use std::io::{BufRead, BufReader};

pub fn diffs(file_name: &String, last_snapshot: &Snapshot) -> Result<(), Error> {
    let current_file = read_lines(file_name)?;

    let snapped_file_path = ".kifi\\".to_string() + &last_snapshot.name + "\\" + file_name;
    let snapped_file = read_lines(&snapped_file_path)?;

    let changes = slice_diff_patch::lcs_diff(&snapped_file, &current_file);
    if changes.is_empty() {
        return Ok(());
    }

    // To debug:
    #[cfg(debug_assertions)]
    println!("{:?}\n", &changes);

    println!("{}", file_name);
    display_diffs(snapped_file, changes)?;

    Ok(())
}

fn read_lines(path: &String) -> Result<Vec<String>, Error> {
    let file = fs::File::open(path).map_err(Error::ReadFile)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.map_err(Error::ReadFile)?);
    }

    Ok(lines)
}

fn display_diffs(
    mut snapped_file: Vec<String>,
    changes: Vec<slice_diff_patch::Change<String>>,
) -> Result<(), Error> {
    let mut line_numbers: Vec<usize> = (1..=snapped_file.len()).collect();

    for change in changes {
        match change {
            slice_diff_patch::Change::Remove(index) => {
                println!(
                    "\x1B[91m- {}\t|{}\x1B[0m",
                    line_numbers.remove(index),
                    snapped_file.remove(index)
                );
            }
            slice_diff_patch::Change::Insert((index, element)) => {
                println!("\x1B[32m+ {}\t|{}\x1B[0m", (&index + 1), &element);

                // Anything can be inserted here, this is just tracking the line number where lines exist.
                // So the index is important, not the element. 0 is just a placeholder.
                // There could be an enum instead, but there really isn't any need for it.
                line_numbers.insert(index, 0);
                snapped_file.insert(index, element);
            }
            slice_diff_patch::Change::Update((index, element)) => {
                println!(
                    "\x1B[91m- {}\t|{}\x1B[0m",
                    line_numbers
                        .get(index)
                        .expect("Diffs were just calculated, this index should exist."),
                    snapped_file
                        .get(index)
                        .expect("Diffs were just calculated, this index should exist.")
                );
                println!("\x1B[32m+ {}\t|{}\x1B[0m", (&index + 1), &element);

                // Setting the element to zero has no use, but it could be helpful while debugging.
                // line_numbers[index] = 0;
                snapped_file[index] = element;
            }
        }
        println!();
    }

    Ok(())
}
