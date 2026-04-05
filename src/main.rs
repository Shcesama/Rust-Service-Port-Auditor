use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::{IpAddr, SocketAddr};
use colored::*;
use futures::stream::{self, StreamExt};
use clap::Parser; // Profesyonel argüman yönetimi

// Terminal argümanlarını tanımlıyoruz
#[derive(Parser)]
#[command(author, version, about = "Rust ile yazılmış Akıllı Ağ Denetçisi")]
struct Args {
    #[arg(short, long)]
    target: String, // Hedef IP

    #[arg(short, long, default_value = "21,22,80,443,445,3306,8080")]
    ports: String, // Taranacak portlar (virgülle ayrılmış)
}

async fn identify_service(mut stream: TcpStream, port: u16) {
    let mut buffer = [0; 512];
    if port == 80 || port == 8080 {
        let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await;
    }

    match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            let cleaned = banner.trim().replace("\r\n", " ");
            println!("   {} Servis: {}", "-->".yellow(), cleaned.cyan());
        }
        _ => println!("   {} Yanıt yok.", "-->".black().bold()),
    }
}

async fn scan_port(ip: IpAddr, port: u16) {
    let addr = SocketAddr::new(ip, port);
    match timeout(Duration::from_millis(800), TcpStream::connect(&addr)).await {
        Ok(Ok(stream)) => {
            println!("{} Port {:>5} {}", "[+]".green().bold(), port, "AÇIK".green());
            identify_service(stream, port).await;
        }
        _ => {}
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let target_ip: IpAddr = args.target.parse().expect("HATA: Geçersiz IP adresi!");
    
    // Virgülle ayrılmış portları sayı listesine çeviriyoruz
    let ports: Vec<u16> = args.ports
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    println!("{} {} üzerinde denetim başlıyor...", "[*]".blue().bold(), target_ip);
    
    stream::iter(ports)
        .for_each_concurrent(5, |port| async move {
            scan_port(target_ip, port).await;
        })
        .await;

    println!("\n{} Denetim tamamlandı.", "[*]".blue().bold());
}