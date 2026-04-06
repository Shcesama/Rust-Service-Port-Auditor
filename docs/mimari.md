# Proje Mimarisi ve Çalışma Mantığı

Bu belge, projenin arka planda nasıl çalıştığını ve kod yapısının nasıl kurgulandığını açıklar.

## 1. Genel Yapı
Program, verilen bir IP adresindeki kapıların (portların) açık olup olmadığını kontrol eden bir ağ aracıdır. Rust programlama dilinin sunduğu performans avantajları kullanılarak, işlemlerin olabildiğince hızlı ve hatasız yapılması hedeflenmiştir.

## 2. Asenkron İşlemler
Ağ üzerindeki kontrollerin yavaşlamaması için **Tokio** kütüphanesi kullanılmıştır. 
* Program, bir portun cevap vermesini beklerken boşta durmaz.
* Eşzamanlı (concurrent) yapı sayesinde aynı anda birden fazla port kontrol edilir.

## 3. Kaynak Yönetimi
Aynı anda çok fazla işlem yapmak bilgisayarı veya ağı yorabilir. Bunu engellemek için kod içinde `Semaphore` adı verilen bir kısıtlayıcı mekanizma bulunur. 
* Bu mekanizma, kullanıcı kaç tane eşzamanlı işleme izin verirse (örneğin 100), aynı anda sadece o kadar işlemin çalışmasını garanti eder. Biri bittiğinde sıradaki işleme başlanır.

## 4. Servis Kontrolü
Bir portun açık olduğu tespit edildiğinde, program o porta standart bir mesaj gönderir. Karşı taraftan gelen cevaba (metin veya veri) bakarak, o portun arkasında hangi yazılımın (HTTP, FTP vb.) çalıştığını anlamaya çalışır ve bunu rapora ekler.