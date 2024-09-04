//sezar şifrelemeye göre şifreleme yapıcaz..anhtar , alfadeki elamanın kaç adım ilerliyceği..

use std::io;

//fonksiyonum şöyle çalışıyor. verilen metni karakter karakter alıyor. buuyk kucuk ve karakter kontrolu yapıp
//sifreleme adımına geciyor. karaketerin ascıı kodunu alıp kaydırma miktarını ekleriz sonrasında collecte
//sifrelenmiş karakterleri birleştirerek string ifadeyi olustururuz
fn sezar_sifreleme(metin: &str, kaydirma: i32) -> String {
    let kaydirma = kaydirma % 26;

    metin.chars()
        .map(|karakter| {
            if karakter.is_ascii_alphabetic() {
                let baslangic = if karakter.is_ascii_lowercase() { 'a' } else { 'A' };
                let kaydirilmis = ((karakter as u8 - baslangic as u8 + kaydirma as u8) % 26 + baslangic as u8) as char;
                kaydirilmis
            } else {
                karakter
            }
        })
        .collect()
}

pub fn run() {
    let mut metin = String::new();
    let mut kaydirma = String::new();

    println!("Şifrelemek istediğiniz metni girin:");
    io::stdin().read_line(&mut metin).expect("Girdi okunamadı");

    println!("Anahtarı girin:");
    io::stdin().read_line(&mut kaydirma).expect("Girdi okunamadı");

    let kaydirma: i32 = kaydirma.trim().parse().expect("Geçersiz sayı girişi");

    let sifrelenmis_metin = sezar_sifreleme(&metin, kaydirma);

    println!("Şifrelenmiş metin: {}", sifrelenmis_metin);
}
