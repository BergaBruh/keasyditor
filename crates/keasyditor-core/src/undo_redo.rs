/// Generic undo/redo stack that stores immutable snapshots.
///
/// Pushing a new state clears any redo history. The stack has no maximum
/// size limit by default.
#[derive(Clone, Debug)]
pub struct UndoRedoStack<T: Clone> {
    /// Past states (most recent at the end). Does not include current.
    undo_stack: Vec<T>,
    /// Future states (most recent undo at the end).
    redo_stack: Vec<T>,
    /// The current state.
    current: T,
}

impl<T: Clone> UndoRedoStack<T> {
    /// Create a new undo/redo stack with the given initial state.
    pub fn new(initial: T) -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current: initial,
        }
    }

    /// Push a new state onto the stack. Clears any redo history.
    pub fn push(&mut self, state: T) {
        self.undo_stack.push(self.current.clone());
        self.current = state;
        self.redo_stack.clear();
    }

    /// Undo the last change, returning the restored state.
    /// Returns `None` if there is nothing to undo.
    pub fn undo(&mut self) -> Option<&T> {
        let prev = self.undo_stack.pop()?;
        self.redo_stack.push(self.current.clone());
        self.current = prev;
        Some(&self.current)
    }

    /// Redo the last undone change, returning the restored state.
    /// Returns `None` if there is nothing to redo.
    pub fn redo(&mut self) -> Option<&T> {
        let next = self.redo_stack.pop()?;
        self.undo_stack.push(self.current.clone());
        self.current = next;
        Some(&self.current)
    }

    /// Get a reference to the current state.
    pub fn current(&self) -> &T {
        &self.current
    }

    /// Get a mutable reference to the current state.
    ///
    /// WARNING: Mutating the current state directly bypasses undo history.
    /// Prefer using `push()` for trackable changes.
    pub fn current_mut(&mut self) -> &mut T {
        &mut self.current
    }

    /// Whether there are any states to undo.
    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    /// Whether there are any states to redo.
    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }

    /// Number of undo steps available.
    pub fn undo_count(&self) -> usize {
        self.undo_stack.len()
    }

    /// Number of redo steps available.
    pub fn redo_count(&self) -> usize {
        self.redo_stack.len()
    }

    /// Reset the stack to a new initial state, clearing all history.
    pub fn reset(&mut self, state: T) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.current = state;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initial_state() {
        let stack = UndoRedoStack::new(42);
        assert_eq!(*stack.current(), 42);
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn push_and_undo() {
        let mut stack = UndoRedoStack::new(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(*stack.current(), 3);
        assert_eq!(stack.undo_count(), 2);

        assert_eq!(stack.undo(), Some(&2));
        assert_eq!(*stack.current(), 2);

        assert_eq!(stack.undo(), Some(&1));
        assert_eq!(*stack.current(), 1);

        assert_eq!(stack.undo(), None);
        assert_eq!(*stack.current(), 1);
    }

    #[test]
    fn undo_and_redo() {
        let mut stack = UndoRedoStack::new(1);
        stack.push(2);
        stack.push(3);

        stack.undo(); // back to 2
        stack.undo(); // back to 1

        assert_eq!(stack.redo(), Some(&2));
        assert_eq!(*stack.current(), 2);

        assert_eq!(stack.redo(), Some(&3));
        assert_eq!(*stack.current(), 3);

        assert_eq!(stack.redo(), None);
    }

    #[test]
    fn push_clears_redo() {
        let mut stack = UndoRedoStack::new(1);
        stack.push(2);
        stack.push(3);

        stack.undo(); // back to 2
        assert!(stack.can_redo());

        stack.push(4); // should clear redo
        assert!(!stack.can_redo());
        assert_eq!(*stack.current(), 4);

        stack.undo();
        assert_eq!(*stack.current(), 2);
    }

    #[test]
    fn reset_clears_all() {
        let mut stack = UndoRedoStack::new(1);
        stack.push(2);
        stack.push(3);
        stack.undo();

        stack.reset(10);
        assert_eq!(*stack.current(), 10);
        assert!(!stack.can_undo());
        assert!(!stack.can_redo());
    }

    #[test]
    fn with_strings() {
        let mut stack = UndoRedoStack::new("hello".to_string());
        stack.push("world".to_string());
        assert_eq!(stack.current(), "world");
        assert_eq!(stack.undo(), Some(&"hello".to_string()));
    }

    #[test]
    fn counts() {
        let mut stack = UndoRedoStack::new(0);
        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.undo_count(), 3);
        assert_eq!(stack.redo_count(), 0);

        stack.undo();
        assert_eq!(stack.undo_count(), 2);
        assert_eq!(stack.redo_count(), 1);

        stack.undo();
        assert_eq!(stack.undo_count(), 1);
        assert_eq!(stack.redo_count(), 2);
    }
}
