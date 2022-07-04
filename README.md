# Lập trình Concurrency với Rust
## Command

    cargo run --example <exm>

## Note

### I. Sử dụng Thread để xử lý đồng thời
---

Thư viện tiêu chuẩn Rust sử dụng mô hình triển khai luồng 1: 1, là một chương trình sử dụng một luồng hệ điều hành trên một luồng ngôn ngữ.

---

Khi luồng chính của một chương trình Rust hoàn thành, tất cả các luồng sinh sản sẽ bị tắt, cho dù chúng đã chạy xong hay chưa.

    cargo run --example exm01

---

Chờ tất cả các chuỗi kết thúc bằng cách sử dụng các `JoinHandle`. Để giải quyết vấn đề (2), chúng ta lưu giá trị trả về của thread:spawn vào một biến số. Nó sẽ mang kiểu dữ liệu là JoinHandle. Khi ghi phương thức `join()` chương trình sẽ đợi đến khi các threads hoàn thành.

    cargo run --example exm02

---

Các chi tiết nhỏ, chẳng hạn như nơi `join()` được gọi, có thể ảnh hưởng đến việc các luồng của bạn có chạy đồng thời hay không.

    cargo run --example exm03

---

sử dụng từ khóa `move` với các `closures` được truyền vào trong `thread :: spawn()`. Khi đó, `closures` sẽ có quyền sở hữu các giá trị mà nó sử dụng từ môi trường, do đó chuyển quyền sở hữu các giá trị đó từ một thread này sang một thread khác.

    cargo run --example exm04

---

### II. Sử dụng Message Passing để chuyền Data giữa các Threads
---
`Channel` là một khái niệm lập trình chung mà dữ liệu được gửi từ luồng này sang luồng khác. Một Channel có hai phần: kênh phát và kênh nhận.Cách thư viện tiêu chuẩn của Rust triển khai các channels là một channel có thể có nhiều đầu gửi tạo ra các giá trị nhưng chỉ một đầu nhận sử dụng các giá trị đó. 
Ví dụ: Khi truyền tx vào trong `closures` thread:spawn và `send()`, là ta đang thực hiện việc `spawned thread` trao đổi thông tin với `main thread`. Điều này giống như đưa một con vịt cao su lên thượng nguồn sông hoặc gửi một tin nhắn trò chuyện từ chủ đề này sang chủ đề khác.

    cargo run --example exm05