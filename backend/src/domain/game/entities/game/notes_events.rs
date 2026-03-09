use super::Game;

impl Game {
    pub fn set_notes(&mut self, notes: String) {
        self.notes = notes;
    }

    pub fn set_hidden_notes(&mut self, hidden_notes: String) {
        self.hidden_notes = hidden_notes;
    }
}