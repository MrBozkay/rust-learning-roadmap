# Async Chat Application

Bu proje, Rust öğrenim yol haritasının dördüncü adımıdır. `tokio` kütüphanesi kullanılarak geliştirilmiş, asenkron (async/await) çalışan gerçek zamanlı bir chat uygulamasıdır.

## Özellikler

- **Asenkron Mimari:** Bloklamayan I/O işlemleri sayesinde yüksek performans.
- **Broadcast Channel:** Mesajların tüm bağlı istemcilere verimli dağıtımı.
- **Tokio Tasks:** Thread yerine hafif siklet görevler (tasks) kullanımı.
- **Çoklu İstemci:** Aynı anda birden fazla istemcinin bağlanıp konuşabilmesi.

## Kurulum ve Çalıştırma

Bu proje bir sunucu ve bir istemci uygulamasından oluşur.

1.  **Sunucuyu Başlatın:**
    ```bash
    cargo run --bin async_chat
    ```

2.  **İstemciyi Başlatın (Farklı bir terminalde):**
    ```bash
    cargo run --bin client
    ```
    *(Birden fazla istemci başlatarak kendi kendinize konuşabilirsiniz.)*

## Teknik Detaylar

### Kullanılan Teknolojiler
- **Tokio:** Rust'ın en popüler asenkron runtime'ı.
- **Tokio Stream & IO:** Asenkron okuma/yazma işlemleri (`AsyncBufReadExt`, `AsyncWriteExt`).
- **Tokio Sync Broadcast:** Çoklu üretici, çoklu tüketici (MPMC) kanal yapısı.

### Öğrenilen Kavramlar
- **Async/Await:** Senkron kod yazar gibi asenkron kod yazma.
- **Tokio Spawn:** Yeni bir asenkron görev başlatma.
- **Select! Makrosu:** Birden fazla asenkron işlemi aynı anda bekleme ve hangisi önce biterse onu işleme.
- **Socket Splitting:** TCP soketini okuma ve yazma parçalarına ayırma.
