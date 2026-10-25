pub struct GameState {
    curr_word: String,
    word_limit: u16,
    correct: u16,
    wrong: u16,
    typed_chars: Vec<char>,
}

impl GameState {
    pub fn curr_word(&self) -> &str {
        &self.curr_word
    }

    pub fn correct(&self) -> u16 {
        self.correct
    }

    pub fn typed_chars(&self) -> &[char] {
        &self.typed_chars
    }
    pub fn new(word_limit: u16, word: &str) -> Self {
        Self {
            curr_word: word.to_string(),
            word_limit,
            correct: 0,
            wrong: 0,
            typed_chars: Vec::new(),
        }
    }
    pub fn type_char(&mut self, character: char) {
        self.typed_chars.push(character);
    }

    pub fn backspace(&mut self) {
        self.typed_chars.pop();
    }

    pub fn is_word_complete(&self) -> bool {
        self.curr_word.chars().count() == self.typed_chars.len()
    }

    pub fn is_word_correct(&self) -> bool {
        self.curr_word.chars().eq(self.typed_chars.iter().copied())
    }

    pub fn finish_word(&mut self) {
        if self.is_word_correct() {
            self.correct += 1;
        }
        else { self.wrong += 1; }
    }

    pub fn start_word(&mut self, word: &str) {
        self.typed_chars.clear();
        self.curr_word = word.to_string();
    }

    pub fn is_finished(&self) -> bool {
        self.correct + self.wrong >= self.word_limit
    }

    pub fn is_char_correct(&self, character: char) -> bool {
        self.curr_word.chars().nth(self.typed_chars.len()) == Some(character)
    }
    
    pub fn completed_words(&self) -> u16 {
        self.correct + self.wrong
    }
}
