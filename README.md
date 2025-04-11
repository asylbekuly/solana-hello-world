## 🔐 Deployment Info

- **Deployed Program ID**:  
  `Bdp9Vus1bgucSnzjTfy3WAkpsh6ZesRoqzt7uaZMBkvR`

- **Wallet Address (payer)**:  
  *(пример, если ты указывал ключ в `solana config get`)*  
  `$(solana address)` или `~/.config/solana/id.json`

> Убедись, что ты скопировал именно тот `Program ID`, который вывел `solana program deploy`.

---

Если хочешь — вот полный шаблон `README.md` для Solana Hello World:

```markdown
# 🚀 Solana Hello World Program

This is a minimal Solana smart contract (program) written in Rust that prints a log message when invoked.

## 📦 Structure

- `src/lib.rs`: program logic (prints `Hello, Solana!`)
- `examples/client.rs`: Rust client to call the program
- `target/deploy/hello_world.so`: compiled program
- `hello_world-keypair.json`: program ID keypair

## 🧪 Local Testing

```bash
solana-test-validator
solana program deploy ./target/deploy/hello_world.so
cargo run --example client
🔐 Deployment Info
Deployed Program ID:
Bdp9Vus1bgucSnzjTfy3WAkpsh6ZesRoqzt7uaZMBkvR

Wallet Address:
$(solana address)

🛠️ Tech Stack
Rust + Cargo

Solana SDK (solana-program, solana-client)

cargo build-sbf, solana-cli

