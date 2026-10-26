use std::collections::VecDeque;

pub struct GameState {
    curr_word: String,
    words_queue: VecDeque<String>,
    word_limit: u16,
    correct: u16,
    wrong: u16,
    typed_chars: Vec<char>,
    overall_chars_counter: u16,
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

    pub fn overall_chars_counter(&self) -> u16 {
        self.overall_chars_counter
    }
    pub fn new(word_limit: u16, word: &str) -> Self {
        Self {
            curr_word: word.to_string(),
            word_limit,
            words_queue: VecDeque::new(),
            correct: 0,
            wrong: 0,
            typed_chars: Vec::new(),
            overall_chars_counter: 0,
        }
    }

    pub fn type_char(&mut self, character: char) {
        self.typed_chars.push(character);
        self.overall_chars_counter += 1;
    }

    pub fn backspace(&mut self) {
        if self.typed_chars.pop().is_some() {
            self.overall_chars_counter -= 1;
        }
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
        } else {
            self.wrong += 1;
        }
    }

    pub fn next_word(&self) -> Option<&str> {
        let remaining = self.word_limit.saturating_sub(self.completed_words());

        if remaining <= 1 {
            return None;
        }

        self.words_queue.front().map(String::as_str)
    }

    pub fn start_word(&mut self, new_preview_word: &str) {
        self.enqueue_word(new_preview_word);
        self.curr_word = self.words_queue.pop_front().unwrap();
        self.typed_chars.clear();
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

    pub fn enqueue_word(&mut self, word: &str) {
        self.words_queue.push_back(word.to_string());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_word(game: &mut GameState, word: &str) {
        for character in word.chars() {
            game.type_char(character);
        }
    }

    #[test]
    fn new_game_has_empty_state() {
        let game = GameState::new(15, "cat");

        assert_eq!(game.curr_word, "cat");
        assert_eq!(game.word_limit, 15);
        assert_eq!(game.correct, 0);
        assert_eq!(game.wrong, 0);
        assert!(game.typed_chars.is_empty());
        assert!(!game.is_finished());
        assert!(!game.is_word_complete());
    }

    #[test]
    fn partial_word_is_not_complete() {
        let mut game = GameState::new(15, "cat");
        type_word(&mut game, "ca");

        assert!(!game.is_word_complete());
        assert!(!game.is_word_correct());
    }

    #[test]
    fn correct_word_updates_score() {
        let mut game = GameState::new(15, "cat");
        type_word(&mut game, "cat");

        assert!(game.is_word_complete());
        assert!(game.is_word_correct());

        game.finish_word();

        assert_eq!(game.correct, 1);
        assert_eq!(game.wrong, 0);
        assert_eq!(game.completed_words(), 1);
    }

    #[test]
    fn wrong_word_updates_score() {
        let mut game = GameState::new(15, "cat");
        type_word(&mut game, "car");

        assert!(game.is_word_complete());
        assert!(!game.is_word_correct());

        game.finish_word();

        assert_eq!(game.correct, 0);
        assert_eq!(game.wrong, 1);
        assert_eq!(game.completed_words(), 1);
    }

    #[test]
    fn backspace_removes_last_char() {
        let mut game = GameState::new(15, "cat");
        type_word(&mut game, "ca");

        game.backspace();

        assert_eq!(game.typed_chars, vec!['c']);
        assert!(!game.is_word_complete());
    }

    #[test]
    fn backspace_on_empty_input_is_safe() {
        let mut game = GameState::new(15, "cat");

        game.backspace();

        assert!(game.typed_chars.is_empty());
        assert_eq!(game.correct, 0);
        assert_eq!(game.wrong, 0);
    }

    #[test]
    fn corrected_typo_can_produce_correct_word() {
        let mut game = GameState::new(15, "cat");
        type_word(&mut game, "cx");

        game.backspace();
        type_word(&mut game, "at");
        game.finish_word();

        assert!(game.is_word_correct());
        assert_eq!(game.correct, 1);
        assert_eq!(game.wrong, 0);
    }

    #[test]
    fn start_word_resets_input_and_keeps_score() {
        let mut game = GameState::new(15, "cat");
        type_word(&mut game, "cat");
        game.finish_word();

        game.start_word("dog");

        assert_eq!(game.curr_word, "dog");
        assert!(game.typed_chars.is_empty());
        assert_eq!(game.correct, 1);
        assert_eq!(game.wrong, 0);
        assert!(!game.is_word_complete());
    }

    #[test]
    fn session_finishes_at_word_limit() {
        let mut game = GameState::new(2, "cat");
        type_word(&mut game, "cat");
        game.finish_word();

        assert!(!game.is_finished());

        game.start_word("dog");
        type_word(&mut game, "dot");
        game.finish_word();

        assert!(game.is_finished());
        assert_eq!(game.completed_words(), 2);
        assert_eq!(game.correct, 1);
        assert_eq!(game.wrong, 1);
    }

    #[test]
    fn next_char_is_checked_at_current_position() {
        let mut game = GameState::new(15, "cat");

        assert!(game.is_char_correct('c'));
        assert!(!game.is_char_correct('a'));

        game.type_char('c');

        assert!(game.is_char_correct('a'));
        assert!(!game.is_char_correct('c'));

        game.backspace();

        assert!(game.is_char_correct('c'));
    }

    #[test]
    fn unicode_word_uses_character_count() {
        let mut game = GameState::new(15, "żaba");
        type_word(&mut game, "żab");

        assert!(!game.is_word_complete());

        game.type_char('a');

        assert!(game.is_word_complete());
        assert!(game.is_word_correct());
    }

    #[test]
    fn queued_word_becomes_current_word() {
        let mut game = GameState::new(3, "cat");
        game.enqueue_word("dog");

        assert_eq!(game.next_word(), Some("dog"));

        type_word(&mut game, "cat");
        game.finish_word();
        game.start_word("bird");

        assert_eq!(game.curr_word(), "dog");
        assert_eq!(game.next_word(), Some("bird"));
        assert!(game.typed_chars().is_empty());

        type_word(&mut game, "dog");
        game.finish_word();
        game.start_word("fish");

        assert_eq!(game.curr_word(), "bird");
        assert_eq!(game.next_word(), None);
    }
}
