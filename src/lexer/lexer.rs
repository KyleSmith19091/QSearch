#[derive(Debug)]
pub struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    pub fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    pub fn from_vec(content: &'a Vec<char>) -> Self {
        Self { content }
    }

    pub fn trim_left(&mut self) {
        while self.content.len() > 0 && self.content[0].is_whitespace() {
            self.content = &self.content[1..];
        }
    }
    
    pub fn eat(&mut self, n: usize) -> &'a[char] {
        let token = &self.content[0..n];
        self.content = &self.content[n..];
        token
    }

    pub fn next_pred<P: FnMut(&char) -> bool>(&mut self, mut predicate: P) -> &'a [char] {
        let mut n = 0;
        while n < self.content.len() && predicate(&self.content[n]) {
            n += 1;
        }
        self.eat(n)
    }

    pub fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None
        }
    
        if self.content[0].is_alphanumeric() {
            return Some(self.next_pred(|token| token.is_alphanumeric()));
        } else if self.content[0].is_numeric() {
            return Some(self.next_pred(|token| token.is_numeric()));
        } 
        Some(self.eat(1))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a[char];

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

