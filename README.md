# Keploy Rust + MongoDB Quickstart 🚀

This repository is a **Rust + MongoDB** sample application integrated with **Keploy** to record and replay API test cases automatically (no manual test writing).

---

## 📌 What this project contains

✅ Rust REST API (CRUD-like Notes app)  
✅ MongoDB Database  
✅ Mongo Express UI (optional)  
✅ Keploy Record + Replay support  
✅ Works in both:
- **Local setup (cargo run + docker mongo)**
- **Docker setup (docker compose)**

---

## 🧩 API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET    | `/`      | Health check |
| POST   | `/notes` | Create a new note |
| GET    | `/notes` | Fetch all notes |

---

## ⚙️ Prerequisites

Make sure you have these installed:

### ✅ 1) Git
**Debian/Ubuntu**
```bash
sudo apt update
sudo apt install -y git
```
Mac

```bash
brew install git
```
✅ 2) Rust (Cargo)

Install Rust using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```
Verify:

```bash
rustc --version
cargo --version
```
✅ 3) Docker + Docker Compose

Debian/Ubuntu

```bash
sudo apt update
sudo apt install -y docker.io docker-compose-plugin
sudo usermod -aG docker $USER
newgrp docker
```
Verify:

```bash
docker --version
docker compose version
```
✅ 4) Keploy

Install Keploy:

```bash
curl --silent --location "https://keploy.io/install.sh" | bash
```
Verify:

```bash
keploy -v
```

📥 Clone the repository
```bash
git clone https://github.com/Gagan202005/Keploy-Rust-Mongodb-QuickStart.git
cd Keploy-Rust-Mongodb-QuickStart
```
## ✅ Local Setup

In local setup:

MongoDB runs in Docker

Rust app runs using cargo run

1️⃣ Start MongoDB (Docker)
```bash
docker compose up -d mongo mongo-express
```
Check running containers:

```bash
docker ps
```
✅ Mongo is available at:

mongodb://localhost:27017

✅ Mongo Express UI:

http://localhost:8081

2️⃣ Run the Rust app
```bash
cargo run
```
You should see:
```bash
🚀 Server running at http://localhost:8000
```
3️⃣ Test the API (cURL Requests)

✅ Health check
```bash
curl -i http://localhost:8000/
```
✅ Create a note
```bash
curl -i -X POST http://localhost:8000/notes \
  -H "Content-Type: application/json" \
  -d '{"text":"hello keploy"}'
```
✅ Get all notes
```bash
curl -i http://localhost:8000/notes
```
🎥 Keploy Recording (Local)

✅ Step 1: Start recording
Run:

```bash
keploy record -c "cargo run"
```
Keploy will start your app and begin recording API calls.

✅ Step 2: Hit APIs (Generate testcases)

Now send requests from another terminal (Mac terminal / different tab):

```bash
curl -i -X POST http://localhost:8000/notes \
  -H "Content-Type: application/json" \
  -d '{"text":"hello keploy"}'
```
```bash
curl -i http://localhost:8000/notes
```
```bash
curl -i http://localhost:8000/
```
✅ You should see logs like:

<img width="1439" height="814" alt="image" src="https://github.com/user-attachments/assets/6fbdbc74-7aa4-4dee-b696-654cc8e1360b" />

✅ Step 3: Stop recording

Press:

```bash
Ctrl + C
```
📂 Recorded testcases location

After recording, Keploy stores testcases here:

```bash
ls keploy/
```
Example:

```bash
keploy/test-set-0/tests
keploy/test-set-0/mocks
```
▶️ Keploy Replay (Local)

Run:

```bash
keploy test -c "cargo run" --delay 10
```
<img width="1440" height="872" alt="image" src="https://github.com/user-attachments/assets/7cae6889-f73e-44c5-bfe7-6bc2ea576c97" />

✅ Expected:

It should replay recorded API testcases

Tests should pass ✅

## 🐳 Docker Setup (Full Docker Compose)

In docker setup:

Mongo runs in Docker

Rust app runs in Docker

1️⃣ Start full application
```bash
docker compose up -d --build
```
Check:

```bash
docker compose ps
```
2️⃣ Test the APIs (Mac/Host Terminal)
```bash
curl -i http://localhost:8000/
```
```bash
curl -i -X POST http://localhost:8000/notes \
  -H "Content-Type: application/json" \
  -d '{"text":"hello docker"}'
```
```bash
curl -i http://localhost:8000/notes
```
3️⃣ Stop docker setup
```bash
docker compose down -v
```
🎥 Keploy Recording (Docker)

✅ Start Mongo first:

```bash
docker compose up -d mongo mongo-express
```
✅ Now record app container using Keploy (foreground mode):

```bash
keploy record -c "docker compose up --build app" --container-name keploy-rust-app
```
Now send API requests (from another terminal):

```bash
curl -i http://localhost:8000/
```
```bash
curl -i -X POST http://localhost:8000/notes \
  -H "Content-Type: application/json" \
  -d '{"text":"hello docker"}'
```
```bash
curl -i http://localhost:8000/notes
```

<img width="1440" height="835" alt="image" src="https://github.com/user-attachments/assets/7307ca81-288c-452c-8ad3-8c70cf711bb8" />

Stop recording using:

```bash
Ctrl + C
```
▶️ Keploy Replay (Docker)

✅ Keep mongo running:

```bash
docker compose up -d mongo mongo-express
```
✅ Replay:

```bash
keploy test -c "docker compose up --build app" --delay 10 --container-name keploy-rust-app
```
<img width="1440" height="872" alt="image" src="https://github.com/user-attachments/assets/bf2e0894-0fee-4ed2-aa71-2982121f8e60" />

🛠 Troubleshooting

❌ Port 8000 already in use

Check:

```bash
sudo lsof -i :8000
```
Kill:

```bash
sudo fuser -k 8000/tcp
```
❌ Mongo not running

Restart:

```bash
docker compose down -v
docker compose up -d mongo mongo-express
```

⭐ If this helped you

Please ⭐ the repo and share feedback!

Happy testing with Keploy 🐰✨

makefile
Copy code
::contentReference[oaicite:0]{index=0}
