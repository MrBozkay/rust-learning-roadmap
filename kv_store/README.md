# Key-Value Database 🗄️

Bu proje, Rust öğrenim yol haritasının beşinci adımıdır. "Bitcask" mimarisinden esinlenerek geliştirilmiş, disk tabanlı, kalıcı bir anahtar-değer (key-value) veritabanıdır.

## 🚀 Özellikler

- **Log-Structured Storage:** Veriler dosyanın sonuna eklenerek (append-only) yazılır. Bu, disk yazma performansını maksimize eder.
- **In-Memory Index:** Anahtarlar ve dosyadaki konumları (offset) RAM'de tutulur (`HashMap`), böylece okuma işlemleri tek bir disk erişimi ile yapılır.
- **Binary Serialization:** Veriler `bincode` kütüphanesi ile binary formatta saklanır, bu da JSON'a göre daha az yer kaplar ve daha hızlıdır.
- **Crash Recovery:** Veritabanı başlatıldığında log dosyası taranarak index yeniden oluşturulur.

## 🛠️ Kurulum ve Çalıştırma

```bash
cd kv_store
cargo run -- --help
```

## 📖 Kullanım

### Veri Ekleme
```bash
cargo run -- set <ANAHTAR> <DEĞER>
# Örnek:
cargo run -- set rust "harika"
```

### Veri Okuma
```bash
cargo run -- get <ANAHTAR>
# Örnek:
cargo run -- get rust
# Çıktı: harika
```

### Veri Silme
```bash
cargo run -- rm <ANAHTAR>
# Örnek:
cargo run -- rm rust
```

## 🏗️ Mimari ve Kod Yapısı

Veritabanı iki ana bileşenden oluşur:

1.  **Log Dosyası (`log.kv`):**
    - Tüm işlemler (`Set`, `Remove`) sırayla bu dosyaya yazılır.
    - Silme işlemi, veriyi dosyadan silmez; bunun yerine bir "Tombstone" (mezar taşı) kaydı ekler.

    ```text
    [SET key1=val1] [SET key2=val2] [REMOVE key1] [SET key2=val3] ...
    ```

2.  **Index (`HashMap<String, u64>`):**
    - Program açılışında log dosyası taranır.
    - Her `Set` işlemi index'i günceller.
    - Her `Remove` işlemi index'ten kaydı siler.
    - Sonuç olarak index, her anahtarın dosyadaki *en güncel* konumunu tutar.

### Kullanılan Kütüphaneler
- **`serde` & `bincode`:** Veri serileştirme.
- **`clap`:** Komut satırı arayüzü.
- **`anyhow`:** Hata yönetimi.
