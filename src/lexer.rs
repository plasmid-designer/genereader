#[derive(Debug)]
struct SourcePosition {
    line: usize,
    column: usize,
    history: Option<Vec<SourcePosition>>,
}

impl SourcePosition {
    fn new() -> Self {
        SourcePosition {
            line: 1,
            column: 0,
            history: Some(Vec::new()),
        }
    }

    fn new_history(source: &SourcePosition) -> Self {
        SourcePosition {
            line: source.line,
            column: source.column,
            history: None,
        }
    }

    fn update(&mut self, c: char) {
        // Log current position in history
        let source_pos = SourcePosition::new_history(self);
        self.history.as_mut().unwrap().push(source_pos);

        // Update line and column
        match c {
            '\n' => {
                self.line += 1;
                self.column = 0;
            }
            _ => self.column += 1,
        }
    }

    fn last(&self) -> Option<&SourcePosition> {
        self.history.as_ref().and_then(|history| history.last())
    }
}

#[derive(Debug)]
struct Lexer<I: Iterator<Item = char>> {
    source: I,
    source_pos: SourcePosition,
}

impl<I> Lexer<I>
where
    I: Iterator<Item = char>,
{
    fn new(source: I) -> Self {
        Lexer {
            source,
            source_pos: SourcePosition::new(),
        }
    }

    fn read_char(&mut self) -> char {
        let c = self.source.next().unwrap();
        self.source_pos.update(c);
        c
    }

    fn read_chars(&mut self, n: usize) -> String {
        let mut s = String::new();
        for _ in 0..n {
            s.push(self.read_char());
        }
        s
    }
}
