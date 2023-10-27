use crate::Error;
use crate::output::Output;
use std::fs;
use std::io::{BufRead, BufReader};

pub fn generate_diffs(snapped_file: Vec<String>, current_file: Vec<String>, output: &mut dyn Output) -> Result<(), Error> {
    let changes = slice_diff_patch::lcs_diff(&snapped_file, &current_file);
    if changes.is_empty() {
        return Ok(());
    }

    // To debug:
    #[cfg(debug_assertions)]
    println!("{:?}\n", &changes);

    generate_output_from_diffs(snapped_file, changes, output)?;
    Ok(())
}

pub fn read_lines(path: &String) -> Result<Vec<String>, Error> {
    let file = fs::File::open(path).map_err(Error::ReadFile)?;
    let reader = BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        lines.push(line.map_err(Error::ReadFile)?);
    }

    Ok(lines)
}

fn generate_output_from_diffs(
    mut snapped_file: Vec<String>,
    changes: Vec<slice_diff_patch::Change<String>>,
    output: &mut dyn Output,
) -> Result<(), Error> {
    let mut line_numbers: Vec<usize> = (1..=snapped_file.len()).collect();

    for change in changes {
        output.add(match change {
            slice_diff_patch::Change::Remove(index) => {
                format!(
                    "\x1B[91m- {}\t|{}\x1B[0m",
                    line_numbers.remove(index),
                    snapped_file.remove(index)
                )
            }
            slice_diff_patch::Change::Insert((index, element)) => {
                // Anything can be inserted here, this is just tracking the line number where lines exist.
                // So the index is important, not the element. 0 is just a placeholder.
                // There could be an enum instead, but there really isn't any need for it.
                line_numbers.insert(index, 0);
                snapped_file.insert(index, element.clone());
                format!("\x1B[32m+ {}\t|{}\x1B[0m", (index + 1), element)
            }
            slice_diff_patch::Change::Update((index, element)) => {
                let removed = snapped_file
                    .get(index)
                    .expect("Diffs were just calculated, this index should exist."
                ).clone();
                format!(
                    "\x1B[91m- {}\t|{}\x1B[0m\n\x1B[32m+ {}\t|{}\x1B[0m",
                    line_numbers
                        .get(index)
                        .expect("Diffs were just calculated, this index should exist."),
                    removed,
                    (&index + 1),
                    {
                        snapped_file[index] = element.clone();
                        element
                    },
                )

                // Setting the element to zero has no use, but it could be helpful while debugging.
                // line_numbers[index] = 0;
                // snapped_file[index] = element;
            }
        });

        output.add(String::from(""));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::generate_diffs;
    use crate::output::{Output, DebugOutput};

    #[test]
    fn test_diffs() {
        let dummy_snapped_file = vec![
            "Lorem ipsum dolor sit amet",
            "consectetur adipiscing elit",
            "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua",
            "Ut enim ad minim veniam",
            "quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat",
            "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur",
            "sunt in culpa qui officia deserunt mollit anim id est laborum",
            "Vulputate ut pharetra sit amet",
        ];

        let dummy_changed_file = vec![
            "Lorem ipsum dolor sit amet",
            "sed do eiusmod tempor incididunt ut labore et dolore magna aliqua",
            "Ut enim ad minim veniam",
            "quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat",
            "Excepteur sint occaecat cupidatat non proident",
            "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur",
            "Tempor orci dapibus ultrices in iaculis nunc sed",
            "Vulputate ut pharetra sit amet",
        ];

        let mut snapped_strings: Vec<String> = Vec::new();
        for &literal in &dummy_snapped_file {
            snapped_strings.push(String::from(literal));
        }

        let mut changed_strings: Vec<String> = Vec::new();
        for &literal in &dummy_changed_file {
            changed_strings.push(String::from(literal));
        }

        let test = vec![
            "\x1B[91m- 2\t|consectetur adipiscing elit\x1B[0m",
            "",
            "\x1B[32m+ 5\t|Excepteur sint occaecat cupidatat non proident\x1B[0m",
            "",
            "\x1B[91m- 7\t|sunt in culpa qui officia deserunt mollit anim id est laborum\x1B[0m\n\x1B[32m+ 7\t|Tempor orci dapibus ultrices in iaculis nunc sed\x1B[0m",
            "",
        ];

        let mut test_strings: Vec<String> = Vec::new();
        for &literal in &test {
            test_strings.push(String::from(literal));
        }

        let mut output = DebugOutput::new();
        assert!(generate_diffs(snapped_strings, changed_strings, &mut output).is_ok());

        assert_eq!(test_strings, output.print().expect("generate_diffs() should have given an output."))

    }
}