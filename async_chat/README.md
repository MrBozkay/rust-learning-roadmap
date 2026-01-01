# Async Chat Application 💬

Bu proje, Rust öğrenim yol haritasının dördüncü adımıdır. `tokio` kütüphanesi kullanılarak geliştirilmiş, asenkron (async/await) çalışan, yüksek performanslı bir TCP chat sunucusu ve istemcisidir.

## 🚀 Özellikler

- **Asenkron I/O:** Binlerce eşzamanlı bağlantıyı tek bir thread üzerinde bile verimli şekilde yönetebilir.
- **Broadcast:** Bir kullanıcının yazdığı mesaj, anında diğer tüm bağlı kullanıcılara iletilir.
- **Tokio Tasks:** Her bağlantı için hafif siklet bir "Task" oluşturulur.

## 🛠️ Kurulum ve Çalıştırma

Bu proje iki parçadan oluşur: Sunucu ve İstemci.

### 1. Sunucuyu Başlatma
```bash
cd async_chat
cargo run --bin async_chat
```
*Sunucu `127.0.0.1:8090` portunu dinlemeye başlar.*

### 2. İstemcileri Başlatma
Farklı terminaller açarak birden fazla istemci başlatabilirsiniz:
```bash
cargo run --bin client
```

## 📖 Kullanım Örneği

**Terminal A (Ali):**
```text
> Merhaba herkese!
```

**Terminal B (Ayşe):**
```text
> Merhaba herkese!  <-- Ali'nin mesajı geldi
> Selam Ali!
```

**Terminal A (Ali):**
```text
> Selam Ali!        <-- Ayşe'nin mesajı geldi
```

## 🏗️ Mimari ve Kod Yapısı

Proje, `tokio`'nun sağladığı asenkron primitifler üzerine kuruludur.

### Sunucu (`src/main.rs`)
- **`TcpListener`**: Asenkron olarak bağlantıları kabul eder.
- **`broadcast::channel`**: Çoklu dağıtım (pub/sub) kanalıdır.
  ```rust
  let (tx, _rx) = broadcast::channel(10);
  ```
- **`tokio::spawn`**: Her yeni bağlantı için yeni bir asenkron görev başlatır.
- **`select!` Makrosu**: Aynı anda hem soketten veri okumayı hem de kanaldan mesaj gelmesini bekler.

### İstemci (`src/bin/client.rs`)
- **`TcpStream`**: Sunucuya bağlanır.
- **`split()`**: Soketi okuma (`Reader`) ve yazma (`Writer`) olarak ikiye ayırır. Bu sayede aynı anda hem mesaj yazıp hem de gelen mesajları okuyabiliriz.
  ```rust
  let (reader, mut writer) = socket.into_split();
  
  // Okuma görevi (Arka planda çalışır)
  tokio::spawn(async move { ... });
  
  // Yazma döngüsü (Ana akış)
  loop { ... }
  ```
