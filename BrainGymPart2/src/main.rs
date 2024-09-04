/*
//nodeyapısını oluşturuyoruz ilk olarak. data deegerimiz ve listenin sonraki elemanını
//gösteren next değerimiz..
struct Node {
    data: i32,
    next: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next: None }
    }
    //eleman eklemesi burada yapılır eger self next some ise litenin sonu kadar gidip eelman ekleriz
    //none ise son düğümdür yeni düğüm oluşturup new datayı işaret ederiz...
    fn append(&mut self, data: i32) {
        match self.next {
            Some(ref mut next) => next.append(data),
            None => self.next = Some(Box::new(Node::new(data))),
        }
    }

    //silme işlemindeki döngümüz şöyle çalışıyor. işlem yapıcagımız düğümü currenta atıyoruz dögü current.nexti
    //gösterene kadar döngü devam eder. data==x sağlanınca silmek istediğimiz değeri buldumuz anlamına gelir.
    //nextin nextini-> none
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

fn main() {

    let mut head = Node::new(12);//liste başlangıcını ve elman atamalarını yapıyoruz..

    head.append(1);
    head.append(5);
    head.append(2);
    head.append(-4);
    head.append(8);


    println!("orijin liste");
    head.print_list();
    println!();

    head.delete(8);

    println!("güncel liste");
    head.print_list();
}*/

mod mod1;
mod mod2;
mod mod3;
mod mod4;

use std::io;

fn main() {
    let mut input = String::new();

    loop {
        println!("\nHangi işlemi çalıştırmak istiyorsunuz?");
        println!("1 - Bağlantılı Liste İşlemleri (Modül 1)");
        println!("2 - Yinelenenleri Kaldıran Bağlantılı Liste (Modül 2)");
        println!("3 - Sezar Şifreleme (Modül 3)");
        println!("4 - Minimum Değerli Yığın İşlemleri (Modül 4)");
        println!("5 - Çıkış");

        input.clear();
        io::stdin().read_line(&mut input).expect("Girdi okunamadı");
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Modül 1: Bağlantılı Liste İşlemleri");
                mod1::run();
            }
            "2" => {
                println!("Modül 2: Yinelenenleri Kaldıran Bağlantılı Liste");
                mod2::run();
            }
            "3" => {
                println!("Modül 3: Sezar Şifreleme");
                mod3::run();
            }
            "4" => {
                println!("Modül 4: Minimum Değerli Yığın İşlemleri");
                mod4::run();
            }
            "5" => {
                println!("Çıkış yapılıyor...");
                break;
            }
            _ => {
                println!("Geçersiz seçim. Lütfen 1 ile 5 arasında bir sayı girin.");
            }
        }
    }
}
