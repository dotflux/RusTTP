# RusTTP 🧠

> **It’s a mini web framework and networking engine** capable of serving HTTP and WebSocket traffic concurrently with extensible middleware and manual control over every packet.

## **It Demonstrates Deep Control Over:**

### 1) Custom HTTP Parsing

- **Splits request lines and headers**

- **Handles GET, POST, etc.**

- **Parses cookies manually**

- **Constructs a custom HttpRequest struct**

### 2) Routing System

- **Maps (method, path) → handler function**

- **Executes matching handler**

- **Returns proper HTTP responses manually**

- **Backbone of web frameworks**

### 3) ThreadPool Concurrency

- **Avoids blocking single threads**

- **Queues incoming connections**

- **Maximizes CPU utilization**

### 4) Websockets

- **Handshake detection (Upgrade: websocket)**

- **Key hashing**

- **Frame reading and sending**

- **Echoing messages**

- **Rate-limiting per user**

- **Bridged HTTP and persistent duplex communication**

### 5) Middleware System

- **Cookie authentication**

- **Session checking**

- **User validation**

- **This is the foundation for pluggable middleware architecture**

### 6) Custom Rate Limiter

- **Tracks message frequency**

- **Prevents spam over WS**

- **Can be extended for IP-based or global throttling**

- **Production Level Safety**

## **What Sets It Apart From Typical HTTP Server**

- **Pure Rust, no framework.**

- **Custom ThreadPool**

- **Manual Websocket Implementation**

- **Manual Hashmap Router**

- **Flexible Middleware Logic**

- **Full Control Over HTTP**

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Built with ❤️ using cutting-edge technologies.**
