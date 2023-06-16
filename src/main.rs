use std::{rc::Rc, borrow::BorrowMut};

struct LinkedList {
    data: u64,
    link: Option<Rc<LinkedList>>
}

impl LinkedList {
    /// Constructor
    pub fn new(data: u64) -> Self {
        
        Self {
            data,
            link: None
        }
    }

    /// Append Values
    pub fn append(&mut self, data: u64) -> Rc<LinkedList> {
        let referen = Rc::new(LinkedList::new(data));
        self.link = Some(referen.clone());

        referen
    }
}

fn main() {
    let mut rc_data = Rc::new(LinkedList::new(69));
    let mut data = rc_data.borrow_mut();
    let segmentos = 15;

    for _ in 0..segmentos {
        rc_data = data.append(rand::random());
        data = rc_data.borrow_mut();
    }
}
