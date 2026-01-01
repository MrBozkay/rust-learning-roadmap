# Minigrep (Grep Clone)

Bu proje, Rust öğrenim yol haritasının ikinci adımıdır. Klasik `grep` komut satırı aracının basitleştirilmiş bir Rust versiyonudur.

## Özellikler

- **Metin Arama:** Bir dosya içinde belirli bir metni arar ve eşleşen satırları ekrana yazdırır.
- **Dosya Okuma:** Büyük dosyaları bile satır satır işleyebilir (iteratorler sayesinde).
- **Esnek Arama:** `IGNORE_CASE` çevre değişkeni ile büyük/küçük harf duyarsız arama yapılabilir.
- **Güvenli:** Rust'ın sahiplik (ownership) ve tip güvenliği özelliklerini kullanır.

## Kurulum ve Çalıştırma

1.  Projeyi derleyin ve çalıştırın:
    ```bash
    cargo run -- "aranacak_metin" dosya_yolu.txt
    ```

## Kullanım Örnekleri

### 1. Standart Arama
```bash
cargo run -- "nobody" poem.txt
```

### 2. Büyük/Küçük Harf Duyarsız Arama
```bash
IGNORE_CASE=1 cargo run -- "NOBODY" poem.txt
```

## Teknik Detaylar

### Kullanılan Teknolojiler
- **std::env:** Komut satırı argümanlarını okumak için.
- **std::fs:** Dosya okuma işlemleri için.
- **Iteratorler:** Bellek verimliliği ve fonksiyonel programlama yaklaşımı için.
- **Unit Tests:** Arama mantığını doğrulamak için TDD (Test Driven Development) yaklaşımı kullanıldı.

### Öğrenilen Kavramlar
- **Lifetime Annotations:** Referansların geçerlilik sürelerini yönetme (`'a`).
- **Error Handling:** `Result` ve `Box<dyn Error>` ile dinamik hata yönetimi.
- **Closures:** `Config::build` içinde iterator adaptörleri kullanımı.
- **Environment Variables:** Çalışma zamanı konfigürasyonu.
