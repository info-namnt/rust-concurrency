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
---
4. Các chi tiết nhỏ, chẳng hạn như nơi `join()` được gọi, có thể ảnh hưởng đến việc các luồng của bạn có chạy đồng thời hay không.

   ```cargo run --example exm03```
---
5. sử dụng từ khóa `move` với các `closures` được truyền vào trong `thread :: spawn()`. Khi đó, `closures` sẽ có quyền sở hữu các giá trị mà nó sử dụng từ môi trường, do đó chuyển quyền sở hữu các giá trị đó từ một thread này sang một thread khác.

   ```cargo run --example exm04```