# Rust Learning Roadmap & Projects 🦀

![Rust Learning Cover](assets/cover.png)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge)
![Status](https://img.shields.io/badge/status-active-success.svg?style=for-the-badge)

Bu depo, Rust programlama dilini sıfırdan ileri seviyeye kadar öğrenmek için oluşturulmuş kapsamlı, proje bazlı bir yol haritasını ve bu harita kapsamında geliştirilen uygulamaları içerir.

## 🗺️ Yol Haritası

### Seviye 1: Temeller ve CLI Araçları (Beginner)
**Odak:** Syntax, Ownership/Borrowing, Temel Veri Tipleri, Kontrol Akışı.

- [x] **[Proje 1: To-Do List CLI](./todo_cli/README.md)**
  - *Açıklama:* Görev ekleme, silme ve listeleme özelliklerine sahip, verileri JSON olarak saklayan bir komut satırı aracı.
  - *Kazanımlar:* `struct`, `enum`, `clap` (CLI argümanları), `serde` (JSON serileştirme).

- [x] **[Proje 2: Minigrep (Grep Clone)](./minigrep/README.md)**
  - *Açıklama:* Dosya içinde metin arama yapan, büyük/küçük harf duyarlılığı seçeneği olan bir `grep` klonu.
  - *Kazanımlar:* Dosya okuma, string işleme, Iteratorler, Lifetime kavramı, Environment Variables.

### Seviye 2: Sistem Programlama ve Ağ (Intermediate)
**Odak:** Traits, Generics, Smart Pointers, Concurrency, Async/Await.

- [x] **[Proje 3: Çok Thread'li Web Server](./hello_server/README.md)**
  - *Açıklama:* Sıfırdan HTTP sunucusu yazımı, Thread Pool implementasyonu.
  - *Kazanımlar:* TCP/IP, Thread yönetimi, `Arc` ve `Mutex`.

- [x] **[Proje 4: Gerçek Zamanlı Chat Uygulaması](./async_chat/README.md)**
  - *Açıklama:* `tokio` kullanarak asenkron chat sunucusu ve istemcisi.
  - *Kazanımlar:* Async/Await, Stream işleme, Ağ programlama.

### Seviye 3: İleri Seviye ve Performans (Advanced)
**Odak:** Unsafe Rust, Macros, FFI, Gömülü Sistemler.

- [x] **[Proje 5: Key-Value Veritabanı](./kv_store/README.md)**
  - *Açıklama:* Log-structured dosya tabanlı veritabanı.
  - *Kazanımlar:* Dosya I/O, Serileştirme (`serde`, `bincode`), Veri Yapıları., Bellek yönetimi.

- [ ] **Proje 6: Minimal İşletim Sistemi Çekirdeği**
  - *Plan:* Bare-metal üzerinde çalışan, ekrana yazı yazan mini kernel.
  - *Kazanımlar:* Düşük seviye donanım kontrolü, Paging, Interrupts.

## 🚀 Nasıl Kullanılır?

Her projenin kendi klasörü altında detaylı bir `README.md` dosyası bulunmaktadır. İlgili projeyi çalıştırmak için klasöre gidip `cargo run` komutunu kullanabilirsiniz.

Örnek (Minigrep için):
```bash
cd minigrep
cargo run -- "aranacak_metin" poem.txt
```

## 📚 Kaynaklar

1.  [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
2.  [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
3.  [Rustlings](https://github.com/rust-lang/rustlings)
4.  [Jon Gjengset (YouTube)](https://www.youtube.com/c/JonGjengset)

---

