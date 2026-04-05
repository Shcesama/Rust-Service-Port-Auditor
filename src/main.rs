use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::{IpAddr, SocketAddr};
use colored::*;
use futures::stream::{self, StreamExt};

// Servis tanımlama ve 9. Madde (Anonymous Login) denemesi
async fn probe_service(mut stream: TcpStream, port: u16) {
    let mut buffer = [0; 1024];
    
    // Bazı servisler (FTP gibi) bağlantı anında selamlama gönderir
    // Bazılarına ise (HTTP gibi) bizim bir şey yazmamız gerekir
    if port == 80 {
        let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await;
    }

    match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            let banner_cleaned = banner.trim().replace("\r\n", " ");
            println!("   {} Servis Bilgisi: {}", "-->".yellow(), banner_cleaned.cyan());

            // 9. MADDE: FTP Anonymous Login Testi
            if port == 21 && banner.contains("220") {
                println!("   {} {} FTP Anonim Giriş Test Ediliyor...", "!!!".red().bold(), "KRİTİK:".red());
                // Buraya ileride anonim giriş kodu eklenebilir
            }
        }
        _ => {
            println!("   {} Servis yanıt vermedi (Sessiz servis).", "-->".black().bold());
        }
    }
}

async fn check_port(ip: IpAddr, port: u16) {
    let addr = SocketAddr::new(ip, port);
    let timeout_duration = Duration::from_millis(800);

    match timeout(timeout_duration, TcpStream::connect(&addr)).await {
        Ok(Ok(stream)) => {
            println!("{} Port {:>5} {}", "[+]".green().bold(), port, "AÇIK".green());
            // Kapı açıksa hemen "Zeka" modülünü çağırıyoruz
            probe_service(stream, port).await;
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() {
    // TEST İÇİN: Eğer internetin varsa "scanme.nmap.org" (45.33.32.156) adresini dene
    // Nmap bu adresi insanların test yapması için açık bırakmıştır.
    let target_ip: IpAddr = "45.33.32.156".parse().expect("Geçersiz IP");
    let ports = vec![21, 22, 80, 443, 3306]; // Bilinen kritik portlar
    
    println!("{} {} üzerinde Akıllı Denetim başlıyor...", "[*]".blue(), target_ip);

    stream::iter(ports)
        .for_each_concurrent(10, |port| async move {
            check_port(target_ip, port).await;
        })
        .await;

    println!("\n{} Tüm denetimler tamamlandı.", "[*]".blue());
}