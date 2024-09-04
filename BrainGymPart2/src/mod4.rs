use std::io;

// burada stack ile kullanıcıdan alınan değerleri tıtıyoruz. min stack ileyse sürekli mininmum değeri
//gümcelliyoruz yoksa program zaman karmaşıklıgı yapısına uymuyor..
struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }
    //burada ekleme işlemini gerçekleştiriyoruz.eklerkende min stacki güncelliyoruz.
    //yeni ekleenen min den küçük veya eşitse güncelliyoruz..

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min_stack.is_empty() || x <= *self.min_stack.last().unwrap() {
            self.min_stack.push(x);
        }
    }
    //top elemanını çıkarıyoruz bu min eleman ise ikisindede silcez..
    fn pop(&mut self) {
        if let Some(top) = self.stack.pop() {
            if top == *self.min_stack.last().unwrap() {
                self.min_stack.pop();
            }
        }
    }

    fn top(&self) -> Option<i32> {
        self.stack.last().copied()
    }

    fn min(&self) -> Option<i32> {
        self.min_stack.last().copied()
    }
}

pub fn run() {
    let mut min_stack = MinStack::new();
    let mut input = String::new();

    loop {
        println!("\nLütfen bir işlem seçin:");
        println!("1 - Eleman ekle (push)");
        println!("2 - Son elemanı çıkar (pop)");
        println!("3 - Tepedeki elemanı göster (top)");
        println!("4 - Minimum elemanı göster (min)");
        println!("5 - Çıkış (exit)");

        input.clear();
        io::stdin().read_line(&mut input).expect("Girdi okunamadı");
        let choice = input.trim();

        match choice {
            "1" => {
                println!("Eklemek istediğiniz sayıyı girin:");
                input.clear();
                io::stdin().read_line(&mut input).expect("Girdi okunamadı");
                if let Ok(num) = input.trim().parse::<i32>() {
                    min_stack.push(num);
                    println!("Eklendi: {}", num);
                } else {
                    println!("Geçersiz sayı girişi.");
                }
            }
            "2" => {
                min_stack.pop();
                println!("Son eleman çıkarıldı.");
            }
            "3" => {
                match min_stack.top() {
                    Some(top) => println!("Yığının tepesindeki eleman: {}", top),
                    None => println!("Yığın boş."),
                }
            }
            "4" => {
                match min_stack.min() {
                    Some(min) => println!("Yığındaki minimum eleman: {}", min),
                    None => println!("Yığın boş."),
                }
            }
            "5" => {
                println!("Çıkış yapılıyor...");
                break;
            }
            _ => {
                println!("Geçersiz seçim. Lütfen 1 ile 5 arasında bir sayı girin.");
            }
        }

        println!("Yığın durumu: {:?}", min_stack.stack);
    }
}
