use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::{IpAddr, SocketAddr};
use colored::*;
use futures::stream::{self, StreamExt};
use clap::Parser;

#[derive(Parser)]
#[command(name = "Rust Auditor", author = "Senin Adın", version = "1.0")]
struct Args {
    #[arg(short, long)]
    target: String,
    #[arg(short, long, default_value = "21,22,80,443,445")]
    ports: String,
}

// 9. MADDE: Gerçek FTP Anonim Giriş Denemesi
async fn test_ftp_anonymous(mut stream: TcpStream) -> bool {
    let mut buffer = [0; 512];
    // 1. Kullanıcı adını gönder
    let _ = stream.write_all(b"USER anonymous\r\n").await;
    let _ = timeout(Duration::from_secs(1), stream.read(&mut buffer)).await;
    
    // 2. Şifreyi gönder (genelde herhangi bir email yeterli)
    let _ = stream.write_all(b"PASS guest@example.com\r\n").await;
    
    match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let res = String::from_utf8_lossy(&buffer[..n]);
            // FTP 230 kodu "Login successful" demektir
            res.contains("230")
        }
        _ => false,
    }
}

async fn probe_service(mut stream: TcpStream, port: u16) {
    let mut buffer = [0; 1024];
    if port == 80 { let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await; }

    match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            println!("   {} Servis: {}", "-->".yellow(), banner.trim().replace("\r\n", " ").cyan());

            // 9. MADDE Kontrolü
            if port == 21 && banner.contains("220") {
                print!("   {} FTP Anonim Giriş Deneniyor... ", "!!!".red().bold());
                if test_ftp_anonymous(stream).await {
                    println!("{}", "BAŞARILI! (ZAFİYET BULDUN)".red().bold().blink());
                } else {
                    println!("{}", "Başarısız (Güvenli)".green());
                }
            }
        }
        _ => println!("   {} Yanıt yok.", "-->".black().bold()),
    }
}

async fn check_port(ip: IpAddr, port: u16) {
    let addr = SocketAddr::new(ip, port);
    if let Ok(Ok(stream)) = timeout(Duration::from_millis(800), TcpStream::connect(&addr)).await {
        println!("{} Port {:>5} {}", "[+]".green().bold(), port, "AÇIK".green());
        probe_service(stream, port).await;
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let target_ip: IpAddr = args.target.parse().expect("Geçersiz IP!");
    let ports: Vec<u16> = args.ports.split(',').filter_map(|p| p.trim().parse().ok()).collect();

    println!("{} {} Denetleniyor...", "[*]".blue().bold(), target_ip);
    stream::iter(ports).for_each_concurrent(5, |port| async move {
        check_port(target_ip, port).await;
    }).await;
    println!("\n{} Tarama bitti.", "[*]".blue().bold());
}