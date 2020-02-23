use std::mem;

pub mod linked_list {

    enum Link {
        Empty,
        More(Box<Node>),
    }

    
    pub struct List {
        head: Link
    }

    type Link = Option<Box<Node>>

    
    struct Node {
        elem: i32,
        next: Link,
    }

}
