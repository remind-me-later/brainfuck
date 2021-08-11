use std::fmt;

#[derive(Debug, Clone)]
pub struct CharPosition {
    line: usize,
    character: usize,
}

impl Default for CharPosition {
    fn default() -> Self {
        Self {
            line: 1,
            character: 1,
        }
    }
}

impl CharPosition {
    fn next_char(&mut self) {
        self.character += 1
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.character = 1;
    }
}

impl fmt::Display for CharPosition {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(
            formatter,
            "line: {}, character: {}",
            self.line, self.character
        )
    }
}

pub struct CharPositionEnumerate<'a> {
    lines: std::str::Lines<'a>,
    chars: std::str::Chars<'a>,
    position: CharPosition,
}

impl<'a> From<&'a str> for CharPositionEnumerate<'a> {
    fn from(input: &'a str) -> Self {
        let mut lines = input.lines();
        let chars = lines.next().unwrap().chars();

        CharPositionEnumerate {
            lines: lines,
            chars: chars,
            position: CharPosition::default(),
        }
    }
}

impl<'a> Iterator for CharPositionEnumerate<'a> {
    type Item = (CharPosition, char);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(c) = self.chars.next() {
            self.position.next_char();
            Some((self.position.clone(), c))
        } else {
            while let Some(line) = self.lines.next() {
                self.position.next_line();
                self.chars = line.chars();

                if let Some(c) = self.chars.next() {
                    return Some((self.position.clone(), c));
                }
            }

            None
        }
    }
}
