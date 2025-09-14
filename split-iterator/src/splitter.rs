// This was a real interview question to a Rust Software Engineer position
// Write a iterator that returns a string splitted by specified delimiter

pub struct Splitter<'input> {
    input: &'input str,
    delimiter: char,
    last_delimiter_index: isize,
}

impl<'input> Splitter<'input> {
    pub fn new(input: &'input str, delimiter: char) -> Self {
        let last_delimiter_index: isize = -1;
        Self {
            input,
            delimiter,
            last_delimiter_index,
        }
    }
}

impl<'input> Iterator for Splitter<'input> {

    type Item = &'input str;

    fn next(&mut self) -> Option<Self::Item> {

        let start_index = (self.last_delimiter_index+1) as usize;
        let len = self.input.len();

        if start_index >= len {
            return None;
        }

        let processing_str = &self.input[start_index..];
        println!("processing slice: {}", processing_str);
        let mut next_delimiter_index = 0;

        for (i, ch) in processing_str.chars().enumerate() {
            if ch == self.delimiter {
                println!("found delim. at {}", i);
                next_delimiter_index = i;
                break;
            }
        }

        if next_delimiter_index == 0 {
            next_delimiter_index = len;
        } else {
            next_delimiter_index += start_index;
        }
        self.last_delimiter_index = next_delimiter_index as isize;

        return Some(&self.input[start_index..next_delimiter_index]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_split() {
        let input = "split test, I hope it passes, added another comma, just for testing";
        let delimiter = ',';

        assert_eq!(input.split(delimiter).collect::<Vec<&str>>(), Splitter::new(input, delimiter).collect::<Vec<&str>>());
    }

    #[test]
    fn test_split_text_without_delimiter() {
        let input = "I have a text here but without any delimiters";
        let delimiter = ',';

        assert_eq!(input.split(delimiter).collect::<Vec<&str>>(), Splitter::new(input, delimiter).collect::<Vec<&str>>());
    }
}