# Rust Learning Roadmap & Projects

Bu depo, Rust programlama dilini sıfırdan ileri seviyeye kadar öğrenmek için oluşturulmuş proje bazlı bir yol haritasını ve bu harita kapsamında geliştirilen uygulamaları içerir.

## Yol Haritası ve Projeler

### Seviye 1: Temeller ve CLI Araçları (Beginner)
Rust'ın temelleri, ownership, borrowing ve temel dosya işlemleri.

- **[Proje 1: To-Do List CLI](./todo_cli/README.md)**
  - Görev ekleme, silme ve listeleme özelliklerine sahip, verileri JSON olarak saklayan bir komut satırı aracı.
  - *Kullanılanlar:* `clap`, `serde`, `serde_json`.

- **[Proje 2: Minigrep (Grep Clone)](./minigrep/README.md)**
  - Dosya içinde metin arama yapan, büyük/küçük harf duyarlılığı seçeneği olan bir `grep` klonu.
  - *Kullanılanlar:* `std::env`, `std::fs`, Iteratorler.

### Seviye 2: Sistem Programlama ve Ağ (Intermediate)
Concurrency, Smart Pointers ve Ağ programlama.

- **Proje 3: Çok Thread'li Web Server** (Planlanıyor)
- **Proje 4: Gerçek Zamanlı Chat Uygulaması** (Planlanıyor)

### Seviye 3: İleri Seviye ve Performans (Advanced)
Unsafe Rust, FFI ve Gömülü Sistemler.

- **Proje 5: Key-Value Veritabanı** (Planlanıyor)
- **Proje 6: Minimal İşletim Sistemi Çekirdeği** (Planlanıyor)

## Nasıl Kullanılır?

Her projenin kendi klasörü altında detaylı bir `README.md` dosyası bulunmaktadır. İlgili projeyi çalıştırmak için klasöre gidip `cargo run` komutunu kullanabilirsiniz.

```bash
cd todo_cli
cargo run -- help
```
