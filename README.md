# IntelliChat

## **⚠️ Disclaimer:**

This project is entirely my creation, developed from scratch, showcasing my knowledge with Tauri Js on Windows and JavasScript, Html and Css.

## 📍 Description

Engage with a conversational interface effortlessly! This project presents a Tauri-based chatbot application for Windows, allowing users to interact with an AI in real-time.

## ⚙️ Technologies Used

- **Tauri**: A framework for building tiny, blazing fast binaries for all major desktop platforms.
- **Rust**: The programming language used for backend logic, ensuring performance and safety.
- **Reqwest**: A Rust library for making HTTP requests.
- **Serde**: A framework for serializing and deserializing Rust data structures efficiently and generically.
- **WebSocket**: Facilitates real-time bi-directional communication between the client and server.

## ✅ Installation Guide

1. **Clone the repository:**

   ```bash
   git clone https://github.com/diegormirhan/IntelliChat.git
   cd IntelliChat
   ```

2. **Install Rust and Tauri dependencies:**

    Follow the setup guide on [Tauri's official setup page](https://tauri.app/v1/guides/getting-started/prerequisites).


3. **Install Npm dependencies:**

   ```bash
   npm install
   ```

4. **Start the server:**

   ```bash
   npm run tauri dev
   ```

6. **Build the application:**

   To compile your application for release:
   ```bash
   npm run tauri build
   ```

## ⚠️ License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
