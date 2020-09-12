pub trait GameState: Sized {
    /// A type which represents the actions which agents may take in the game.
    ///
    /// Note that actions may or may not be legal, depending on the state.
    type Action;
    /// A type which represents the different agents in the game
    type Agent;

    /// Returns the next agent to move (or None, if the game has ended)
    fn next_agent(&self) -> Option<Self::Agent>;

    /// Returns true if this game has been finished.
    fn is_finished(&self) -> bool {
        self.next_agent().is_none()
    }

    /// Returns the state which results from the given action, if that action is legal.
    fn successor(&self, action: Self::Action) -> Option<Self>;

    /// Returns whether or not the given action is legal
    fn is_legal(&self, action: Self::Action) -> bool {
        self.successor(action).is_some()
    }
}
