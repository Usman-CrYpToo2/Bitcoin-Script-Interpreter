// defines the Stack data structure and its operations
#[derive(Debug, Clone, PartialEq)]
pub struct Stack {
   pub items : Vec<Vec<u8>>
}

impl Stack {
    pub fn new() -> Stack {
        Stack { items: Vec::new()}
    }

    pub fn push(&mut self, value: Vec<u8>) {
        self.items.push(value);
    }

    pub fn pop(&mut self) -> Option<Vec<u8>> {
        self.items.pop()
    }
    
    pub fn len(&self) -> usize {
         self.items.len()
    }
}


// #[cfg(test)]
// mod test {
//     use std::vec;

// use super::*;

//     #[test]
//     fn push_and_pop_return_same_value() {
//         let mut st = Stack::new();
//         let mut vec : Vec<u8> = vec::Vec::new();
//         vec.push(100);
//         st.push(vec);
       
//     }

    // #[test]
    // fn pop_on_empty_stack() {
    //      let mut st = Stack::new();
    //      assert_eq!(st.pop(), None); 
    // }

    // #[test]
    // fn last_in_first_out() {
    //     let mut stack = Stack::new() ;
    //      stack.push(vec![100]);
    //      stack.push(vec![101]);
    //      stack.push(vec![102]);


    //      assert_eq!(stack.pop(), Some(vec![102]));
    //      assert_eq!(stack.pop(), Some(vec![101]));
    //      assert_eq!(stack.pop(), Some(vec![100]));
    // }


    // #[test]
    // fn push_element_matches_len() {
    //      let mut st = Stack::new();
    //      st.push(vec![100]);
    //      st.push(vec![100]);

    //      assert_eq!(st.len(), 2);

    //      st.pop();
         
    //      assert_eq!(st.len(), 1);

    //      st.pop();

    //      assert_eq!(st.len(), 0);
    // }

// }