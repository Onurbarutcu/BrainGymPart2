use std::collections::HashSet;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::{self, Write};

#[derive(Debug, Clone)]
struct ListNode {
    value: i32,
    next: Option<Rc<RefCell<ListNode>>>,//sahiplikte sorun çıkardıgı için böyle tanımlamak gerekiyormus
}                                       // diğer birden fazla düğüm aynı referansı kullanamaıyo.

impl ListNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(ListNode { value, next: None }))
    }
}

//yinelenen elemanları burada kaldırıyoruz..
fn remove_duplicates(head: Option<Rc<RefCell<ListNode>>>) -> Option<Rc<RefCell<ListNode>>> {
    if head.is_none() {
        return head;
    }

    let mut current = head.clone();
    let mut prev: Option<Rc<RefCell<ListNode>>> = None;
    let mut seen = HashSet::new();//daha önceden görülmüş değerleri tutması için.

    //döngüyle diizdeki bütün elemanları geziyoz. node valueyle mevcut düğüm değerlerini alıyoz
    //dha önce  seene eklendiyse atlayıp next ine bağlıyoz.
    while let Some(node) = current.clone() {
        let node_value = node.borrow().value;

        if seen.contains(&node_value) {
            if let Some(prev_node) = prev.clone() {
                prev_node.borrow_mut().next = node.borrow().next.clone();
            }
        } else {
            seen.insert(node_value);
            prev = current.clone();
        }

        current = node.borrow().next.clone();
    }

    head
}

fn print_linked_list(head: Option<Rc<RefCell<ListNode>>>) {
    let mut current = head;
    while let Some(node) = current {
        print!("{}", node.borrow().value);
        current = node.borrow().next.clone();
        if current.is_some() {
            print!(" => ");
        }
    }
    println!();
}

fn create_linked_list(elements: Vec<i32>) -> Option<Rc<RefCell<ListNode>>> {
    let mut head: Option<Rc<RefCell<ListNode>>> = None;
    let mut current: Option<Rc<RefCell<ListNode>>> = None;

    for element in elements.into_iter().rev() {
        let new_node = ListNode::new(element);
        new_node.borrow_mut().next = head.clone();
        head = Some(new_node.clone());
        if current.is_none() {
            current = Some(new_node);
        }
    }

    head
}

pub fn run() {
    let mut input = String::new();

    println!("liste elemanlarını bir boşluklu girin örneğin: 1 2 3");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Girdi okunamadı");

    // Kullanıcı girdisini işleme ve sayılar dizisine dönüştürme
    let elements: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    if elements.is_empty() {
        println!("Geçersiz giriş sayi girin");
        return;
    }

    let head = create_linked_list(elements);
    println!("orijin liste:");
    print_linked_list(head.clone());

    let head = remove_duplicates(head);
    println!("güncel liste:");
    print_linked_list(head);
}
