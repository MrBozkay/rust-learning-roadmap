# To-Do List CLI

Bu proje, Rust öğrenim yol haritasının başlangıç seviyesi (Beginner) projesidir. Temel Rust kavramlarını pekiştirmek amacıyla geliştirilmiş, komut satırı üzerinden çalışan bir görev yöneticisidir.

## Özellikler

- **Görev Ekleme:** `add` komutu ile yeni görevler oluşturma.
- **Listeleme:** `list` komutu ile mevcut görevleri ve durumlarını görüntüleme.
- **Tamamlama:** `complete` komutu ile görevleri tamamlandı olarak işaretleme.
- **Silme:** `delete` komutu ile görevleri silme.
- **Kalıcılık:** Veriler JSON formatında (`tasks.json`) saklanır, uygulama kapansa bile kaybolmaz.

## Kurulum ve Çalıştırma

Bu projeyi çalıştırmak için sisteminizde Rust ve Cargo yüklü olmalıdır.

1.  Projeyi derleyin ve çalıştırın:
    ```bash
    cargo run -- help
    ```

## Kullanım Örnekleri

### 1. Yeni Görev Ekleme
```bash
cargo run -- add "Rust öğren"
```

### 2. Görevleri Listeleme
```bash
cargo run -- list
```
*Çıktı:* `1 [ ] - Rust öğren`

### 3. Görevi Tamamlama
```bash
cargo run -- complete 1
```

### 4. Görevi Silme
```bash
cargo run -- delete 1
```

## Teknik Detaylar

### Kullanılan Teknolojiler
- **Rust:** Ana programlama dili.
- **Clap:** Komut satırı argümanlarını (CLI) işlemek için.
- **Serde & Serde JSON:** Verileri JSON formatına serileştirmek ve deserileştirmek için.

### Veri Yapıları
- **Task:** Her bir görevi temsil eden yapı (id, açıklama, durum).
- **TodoList:** Görev listesini yöneten ve dosya işlemlerini (load/save) yapan yapı.

### Öğrenilen Kavramlar
- `struct` ve `enum` tanımlama.
- `impl` blokları ile metot tanımlama.
- `Result` tipi ile hata yönetimi.
- Dosya okuma/yazma işlemleri (`std::fs`).
- Sahiplik (Ownership) ve Borçlanma (Borrowing) kuralları.
