# Multi-Threaded Web Server

Bu proje, Rust öğrenim yol haritasının üçüncü adımıdır. Rust'ın standart kütüphanesini (`std::net`, `std::thread`, `std::sync`) kullanarak sıfırdan yazılmış, çok iş parçacıklı (multi-threaded) bir web sunucusudur.

## Özellikler

- **Thread Pool:** Sabit sayıda thread oluşturarak sistem kaynaklarını verimli kullanır.
- **Concurrency:** Uzun süren işlemler (örn. `/sleep`) diğer istekleri bloklamaz.
- **HTTP 1.1:** Temel HTTP protokolü desteği (GET istekleri).
- **Graceful Shutdown:** `Drop` trait'i sayesinde sunucu kapanırken kaynakları temizler.

## Kurulum ve Çalıştırma

1.  Projeyi derleyin ve çalıştırın:
    ```bash
    cargo run
    ```
2.  Tarayıcınızda `http://127.0.0.1:7878` adresine gidin.

## Test Endpoints

- `/`: Ana sayfa (`hello.html`).
- `/sleep`: 5 saniye süren işlem (Concurrency testi için).
- `/herhangi-bir-sey`: 404 sayfası (`404.html`).

## Teknik Detaylar

### Kullanılan Teknolojiler
- **TcpListener:** Gelen TCP bağlantılarını dinlemek için.
- **Arc & Mutex:** Threadler arasında güvenli veri paylaşımı (`mpsc::Receiver` paylaşımı) için.
- **mpsc (Multi-producer, single-consumer):** Ana thread'den işçi thread'lere görev göndermek için kanal yapısı.
- **Box<dyn FnOnce() + Send + 'static>:** Görevleri (closure) tiplemek ve saklamak için.

### Öğrenilen Kavramlar
- **Interior Mutability:** `RefCell` yerine `Mutex` kullanarak thread-safe mutability.
- **Message Passing:** Threadler arası iletişim.
- **Trait Objects:** Dinamik dispatch ile farklı closure tiplerini saklama.
