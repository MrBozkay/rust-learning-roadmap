# To-Do List CLI 📝

Bu proje, Rust öğrenim yol haritasının başlangıç seviyesi (Beginner) projesidir. Temel Rust kavramlarını pekiştirmek amacıyla geliştirilmiş, komut satırı üzerinden çalışan, verileri JSON formatında saklayan bir görev yöneticisidir.

## 🚀 Özellikler

- **Görev Ekleme:** `add` komutu ile yeni görevler oluşturma.
- **Listeleme:** `list` komutu ile mevcut görevleri ve durumlarını görüntüleme.
- **Tamamlama:** `complete` komutu ile görevleri tamamlandı olarak işaretleme.
- **Silme:** `delete` komutu ile görevleri silme.
- **Kalıcılık:** Veriler `tasks.json` dosyasında saklanır.

## 🛠️ Kurulum ve Çalıştırma

```bash
# Proje dizinine girin
cd todo_cli

# Yardım menüsünü görüntüleyin
cargo run -- --help
```

## 📖 Kullanım Senaryoları

### 1. Yeni Görev Ekleme
```bash
cargo run -- add "Rust öğren"
cargo run -- add "Proje yap"
```
**Çıktı:**
```text
Task added: 1
Task added: 2
```

### 2. Görevleri Listeleme
```bash
cargo run -- list
```
**Çıktı:**
```text
1 [ ] - Rust öğren
2 [ ] - Proje yap
```

### 3. Görevi Tamamlama
```bash
cargo run -- complete 1
```
**Çıktı:**
```text
Task 1 completed.
```

### 4. Son Durumu Görme
```bash
cargo run -- list
```
**Çıktı:**
```text
1 [x] - Rust öğren
2 [ ] - Proje yap
```

## 🏗️ Kod Yapısı

Proje tek bir `main.rs` dosyasından oluşsa da, mantıksal olarak modüllere ayrılmıştır:

### Veri Yapıları (`structs`)
- **`Task`**: Tek bir görevi temsil eder.
  ```rust
  struct Task {
      id: usize,
      description: String,
      completed: bool,
  }
  ```
- **`TodoList`**: Görev listesini yönetir ve dosya işlemlerinden sorumludur.
  ```rust
  struct TodoList {
      tasks: Vec<Task>,
  }
  ```

### CLI Yönetimi (`clap`)
- **`Cli`**: Komut satırı argümanlarını parse eder.
- **`Commands`**: `Add`, `List`, `Complete`, `Delete` gibi alt komutları (`enum`) tanımlar.

### Veri Saklama (`serde`)
- `serde` ve `serde_json` kütüphaneleri kullanılarak `TodoList` yapısı JSON formatına çevrilip `tasks.json` dosyasına yazılır.
