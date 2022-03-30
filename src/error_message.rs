use crate::token::SourceSpan;

pub struct ErrorMessage {
    pub filepath: String,
    pub content: String,
    pub pos: usize,
    pub len: usize,
    pub line: usize,
    pub message: String,
}

impl ErrorMessage {
    pub fn new(filepath: String, content: String, message: String, pos: usize) -> ErrorMessage {
        let line = ErrorMessage::line(&content, pos);
        let len = content.len();

        ErrorMessage {
            filepath,
            content,
            pos,
            line,
            len,
            message,
        }
    }

    pub fn print(self) {
        let content = self.content.clone();
        let first_nearest_newline_tokenspan = self.find_nearest_newline_left(1);
        let mut nearest_newline_left_pos = first_nearest_newline_tokenspan.to;
        let nearest_newline_right_pos = self.find_nearest_newline_right(3);
        let start_pos = self.find_nearest_newline_left(3);
        let current_line_number_as_string = (self.line + 1).to_string();
        let current_line_column = self.pos - nearest_newline_left_pos + 1;
        let mut partial_content = content[start_pos.to..nearest_newline_right_pos.to]
            .to_string()
            .split("\n")
            .into_iter()
            .enumerate()
            .map(|(i, x)| {
                format!(
                    "{:>5} | {}",
                    if i == (if self.line > 2 { 2 } else { self.line }) {
                        current_line_number_as_string.clone()
                    } else {
                        "".to_string()
                    },
                    x
                )
                .to_string()
            })
            .collect::<Vec<_>>();
        let mut pointer_line = "      | ".to_string();
        let pointer_line_end_pos = self.pos;

        while nearest_newline_left_pos < pointer_line_end_pos {
            pointer_line.push_str("-");
            nearest_newline_left_pos += 1;
        }

        pointer_line.push_str("^");

        partial_content.insert(
            if self.line > 2 { 3 } else { self.line + 1 },
            format!("\x1b[93m{}\x1b[0m", pointer_line),
        );

        let partial_content = partial_content.join("\n");

        println!("");
        println!(
            "{}:{}:{}",
            self.filepath, current_line_number_as_string, current_line_column
        );
        println!("");
        println!("{}", partial_content);
        println!("      | ");
        println!("");

        println!("\x1b[91mError\x1b[0m: {}", self.message);
        println!("");
    }

    fn line(content: &String, pos: usize) -> usize {
        let mut chars = content.chars().enumerate();
        let mut line = 0;

        while let Some((curr_pos, c)) = chars.next() {
            if curr_pos >= pos {
                break;
            }

            if c == '\n' {
                line += 1;
            }
        }

        line
    }

    fn find_nearest_newline_left(&self, count: usize) -> SourceSpan {
        let mut rev_content = self.content.chars().rev().enumerate();
        let target_pos = self.pos;
        let mut found_counter = 0;
        let mut line = self.line;
        let mut result = SourceSpan::new(0, 0);

        while let Some((curr_pos, c)) = rev_content.next() {
            let curr_pos = (self.len - 1) - curr_pos;

            if found_counter == count {
                break;
            }

            if curr_pos > target_pos {
                continue;
            }

            if c == '\n' {
                found_counter += 1;
                result = SourceSpan::new(curr_pos, curr_pos + 1);
                if line > 0 {
                    line -= 1;
                }
            }
        }

        if found_counter < count {
            return SourceSpan::new(0, 0);
        }

        result
    }

    fn find_nearest_newline_right(&self, count: usize) -> SourceSpan {
        let mut rev_content = self.content.chars().enumerate();
        let target_pos = self.pos;
        let mut found_counter = 0;
        let len = self.len;
        let mut result = SourceSpan::new(len - 1, len);

        while let Some((curr_pos, c)) = rev_content.next() {
            if curr_pos < target_pos {
                continue;
            }

            if curr_pos == self.len {
                break;
            }

            if found_counter == count {
                break;
            }

            if c == '\n' {
                found_counter += 1;
                result = SourceSpan::new(curr_pos, curr_pos + 1);
            }
        }

        if found_counter < count {
            return SourceSpan::new(len - 1, len);
        }

        result
    }
}
