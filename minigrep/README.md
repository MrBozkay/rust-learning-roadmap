# Minigrep (Grep Clone) 🔍

Bu proje, Rust öğrenim yol haritasının ikinci adımıdır. Klasik `grep` komut satırı aracının basitleştirilmiş bir Rust versiyonudur. Dosya okuma, string işleme ve hata yönetimi konularına odaklanır.

## 🚀 Özellikler

- **Metin Arama:** Dosya içinde aranan kelimeyi bulur ve ilgili satırları basar.
- **Büyük/Küçük Harf Duyarlılığı:** `IGNORE_CASE` çevre değişkeni ile kontrol edilebilir.
- **Modüler Yapı:** `main.rs` ve `lib.rs` ayrımı ile temiz kod mimarisi.
- **Test Edilebilir:** İş mantığı unit testlerle doğrulanmıştır.

## 🛠️ Kurulum ve Çalıştırma

```bash
cd minigrep
```

## 📖 Kullanım Senaryoları

Örnek dosya (`poem.txt`) içeriği:
```text
I'm nobody! Who are you?
Are you nobody, too?
How dreary to be somebody!
```

### 1. Standart Arama (Case Sensitive)
"to" kelimesini arayalım (küçük harf):
```bash
cargo run -- "to" poem.txt
```
**Çıktı:**
```text
Are you nobody, too?
How dreary to be somebody!
```

### 2. Büyük/Küçük Harf Duyarsız Arama (Case Insensitive)
"TO" kelimesini arayalım, ancak `IGNORE_CASE` aktif olsun:
```bash
IGNORE_CASE=1 cargo run -- "TO" poem.txt
```
**Çıktı:**
```text
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

## 🏗️ Kod Yapısı

Proje, "Binary" ve "Library" olarak ikiye ayrılmıştır (Separation of Concerns):

### `src/main.rs` (Binary Crate)
- Programın giriş noktasıdır.
- Argümanları alır ve `lib.rs` içindeki `run` fonksiyonunu çağırır.
- Hata oluşursa kullanıcıya anlamlı bir mesaj gösterip çıkar.

### `src/lib.rs` (Library Crate)
- **`Config` Struct:** Argümanları (sorgu, dosya yolu, ignore_case) ayrıştırır ve tutar.
- **`run` Fonksiyonu:** Dosyayı okur ve arama işlemini yönetir.
- **`search` Fonksiyonu:**
  ```rust
  pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
      contents
          .lines()
          .filter(|line| line.contains(query))
          .collect()
  }
  ```
  *Not: Iteratorler (`filter`, `collect`) kullanılarak bellek verimliliği sağlanmıştır.*

### Testler
- `lib.rs` içinde TDD (Test Driven Development) yaklaşımıyla yazılmış unit testler bulunur.
