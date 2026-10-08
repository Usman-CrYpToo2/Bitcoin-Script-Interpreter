//! The script main stack.

use crate::error::ScriptError;

/// A stack of byte vectors. Index 0 is the bottom.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stack {
    items: Vec<Vec<u8>>,
}

impl Stack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, value: Vec<u8>) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Result<Vec<u8>, ScriptError> {
        self.items.pop().ok_or(ScriptError::StackUnderflow)
    }

    pub fn top(&self) -> Option<&[u8]> {
        self.items.last().map(Vec::as_slice)
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn as_slice(&self) -> &[Vec<u8>] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pop_on_empty_stack_underflows() {
        assert_eq!(Stack::new().pop(), Err(ScriptError::StackUnderflow));
    }

    #[test]
    fn last_in_first_out() {
        let mut stack = Stack::new();
        stack.push(vec![1]);
        stack.push(vec![2]);
        stack.push(vec![3]);

        assert_eq!(stack.len(), 3);
        assert_eq!(stack.top(), Some(&[3u8][..]));
        assert_eq!(stack.pop(), Ok(vec![3]));
        assert_eq!(stack.pop(), Ok(vec![2]));
        assert_eq!(stack.pop(), Ok(vec![1]));
        assert!(stack.is_empty());
    }
}
