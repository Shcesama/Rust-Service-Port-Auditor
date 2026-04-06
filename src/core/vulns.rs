use colored::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};

/// FTP servisine şifresiz ve isimsiz (anonymous) olarak giriş yapılıp yapılamadığını kontrol eder.
///
/// # Argümanlar
/// * `stream` - Karşı sistemle kurulan aktif TCP bağlantısı.
///
/// # Dönüş Değeri
/// * Eğer isimsiz giriş başarılı olursa `true`, aksi halde `false` döndürür.
pub async fn test_ftp_anonymous(mut stream: TcpStream) -> bool {
    let mut buffer = [0; 512];

    // 1. Kullanıcı adı gönderimi
    if let Err(e) = stream.write_all(b"USER anonymous\r\n").await {
        eprintln!("{} FTP kullanıcı adı gönderilemedi: {}", "[!]".yellow(), e);
        return false;
    }

    // 2. Sunucudan yanıt bekleme (Eski let _ = yerine hata kontrollü hali)
    if let Err(_) = timeout(Duration::from_secs(1), stream.read(&mut buffer)).await {
        eprintln!(
            "{} FTP sunucusundan yanıt alınamadı (Zaman aşımı).",
            "[!]".yellow()
        );
        return false;
    }

    // 3. Şifre gönderimi (Eski let _ = yerine hata kontrollü hali)
    if let Err(e) = stream.write_all(b"PASS guest@example.com\r\n").await {
        eprintln!("{} FTP şifresi gönderilemedi: {}", "[!]".yellow(), e);
        return false;
    }

    // TODO: Zaman aşımı süresi dışarıdan parametre olarak alınacak.
    // 4. Giriş başarılı mı kontrolü
    if let Ok(Ok(n)) = timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        let res = String::from_utf8_lossy(&buffer[..n]);
        return res.contains("230");
    }

    false
}
