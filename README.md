# Rust Concurrency Programing
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

---

Chúng ta cũng có thể `send` nhiều giá trị một lúc

    cargo run --example exm06

Ở kênh gửi, khi chúng ta gửi lần lượt các thông điệp
Ở kênh nhận, chúng ta có thể nhận các thông điệp thông qua 1 vòng lặp

Ta có thể thấy chúng ta không có bất kỳ dòng code nào cho việc `pause` hoặc `delay` main thread, điều đó có nghĩa `main thread` đang đợi để nhận các thông điệp từ `spawn thread`

---

Nhân bản kênh gửi bằng cách sử dụng `clone`
Trước khi gọi `thread:spawn` chúng ta nhân bản kênh gửi `tx` → `tx1`
Chúng ta chuyển kênh gửi `tx1` cho một `thread` thứ 2. Điều này cung cấp cho chúng ta 2 `thread`, mỗi `thread` sẽ gửi các thông điệp khác nhau đến kênh nhận `rx`.

    cargo run --example exm07

### III. Share state concurrency

---

Sử dụng Mutexes để cho phép truy cập vào dữ liệu từ một thread tại một thời điểm. 

Mutexes có tiếng là khó sử dụng vì bạn phải nhớ hai quy tắc:

- Bạn phải được mở khóa để lấy được dữ liệu 
- Khi bạn sử dụng xong dữ liệu, bạn phải mở khóa để các luồng khác có thể truy cập 

---

Sử dụng Mutex trong đơn luồng 

    cargo run --example exm08

Với nhiều kiểu dữ liệu khác nhau, đều có thể sử dụng `Mutex<T>` kèm với phương thức `new()` để tạo ra 1 Mutex

Để truy cập được Data trong Mutex phải dùng hàm `lock()`. 
Nếu như có 1 thread đang giữ khóa này, hàm `lock()` sẽ trả về `panic`.
Có thể nói `Mutex<T>` là 1 `smart pointer`.

---

Để chia sẻ `Mutex` với nhiều threads khác nhau, Rust sẽ thông báo về việc không thể thay đổi các ownership giữa các thread.

Vì vậy chúng ta cần sử dụng 1 thứ gì đó giống như `Referrence Counter Smart Pointer` để chia sẻ ownership. 

Tuy nhiên `Rc<T>` không giúp ích gì trong trường hợp này vì `Rc<T>` không an toàn khi chia sẻ giữa các threads 

Ở đây chúng ta sẽ đề cập đến `Atomic Referrence Counter` _ Arc<T> 
(Sẽ có đánh đổi về hiệu suất, nên chỉ thực hiện khi bạn thực sự cần)


    cargo run --example exm09