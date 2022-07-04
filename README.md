# Lập trình Concurrency với Rust
## Command
```cargo run --example <exm>```
## Note
---
1. Thư viện tiêu chuẩn Rust sử dụng mô hình triển khai luồng 1: 1, là một chương trình sử dụng một luồng hệ điều hành trên một luồng ngôn ngữ.
---
2. Khi luồng chính của một chương trình Rust hoàn thành, tất cả các luồng sinh sản sẽ bị tắt, cho dù chúng đã chạy xong hay chưa.

    ```cargo run --example exm01```
---
3. Chờ tất cả các chuỗi kết thúc bằng cách sử dụng các `JoinHandle`. Để giải quyết vấn đề (2), chúng ta lưu giá trị trả về của thread:spawn vào một biến số. Nó sẽ mang kiểu dữ liệu là JoinHandle. Khi ghi phương thức `join()` chương trình sẽ đợi đến khi các threads hoàn thành.

   ```cargo run --example exm02```