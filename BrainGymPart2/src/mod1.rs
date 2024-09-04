struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next: None }
    }

    fn append(&mut self, data: i32) {
        match self.next {
            Some(ref mut next) => next.append(data),
            None => self.next = Some(Box::new(Node::new(data))),
        }
    }

    fn delete(&mut self, x: i32) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            if next_node.data == x {
                current.next = next_node.next.take();
                return;
            }
            current = current.next.as_mut().unwrap();
        }
    }

    fn print_list(&self) {
        print!("{} ", self.data);
        if let Some(ref next) = self.next {
            next.print_list();
        }
    }
}

pub fn run() {
    let mut head = Node::new(12);

    head.append(1);
    head.append(5);
    head.append(2);
    head.append(-4);
    head.append(8);

    println!("Orijinal liste:");
    head.print_list();
    println!();

    //silicemiz elemanı burda belirtiyoz..
    head.delete(5);

    println!("Güncel liste:");
    head.print_list();
}
