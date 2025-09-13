# JSON-to-CSV Converter CLI Tool

A simple command-line tool written in **Rust** to convert JSON files into CSV format.  
Supports:  
- ✅ Single JSON objects  
- ✅ Arrays of JSON objects  
- ✅ Nested objects (flattened as `parent.child`)  
- ✅ Arrays inside JSON (joined with `;`)  
- ✅ Null values (written as empty cells)

---

## 🚀 Usage

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run -- <input.json> <output.csv>
```

### Example
#### Input (`input.json`)
```json
{
  "id": 101,
  "name": "Anurag",
  "skills": ["Rust", "Python"],
  "contact": {
    "email": "anurag@example.com",
    "phone": "+91-9876543210"
  }
}
```

#### Output (`output.csv`)
```csv
id,name,skills,contact.email,contact.phone
101,"Anurag","Rust;Python","anurag@example.com","+91-9876543210"
```

---

## ⚙️ Features
- Preserves JSON key order (with `serde_json`’s `preserve_order` feature).  
- Flattens nested objects using dot notation (`contact.email`).  
- Converts arrays to a single CSV cell (`Rust;Python`).  
- Null values become empty cells.  

---

## ⚠️ Limitations
- Handles only **one level of nested objects** (`contact.email`). Deeply nested structures may not flatten completely.  
- Large JSON files are fully loaded into memory (no streaming). May be inefficient for very big files.  
- Arrays of objects inside fields are not expanded into separate rows; they are converted into a single `;`-joined string.  
- Special CSV formatting (escaping commas/quotes inside nested structures) may need improvements for complex cases.  

---

## 📦 Dependencies
- [serde](https://crates.io/crates/serde)  
- [serde_json](https://crates.io/crates/serde_json) (with `preserve_order` feature)  

Add this to your `Cargo.toml`:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0", features = ["preserve_order"] }
```

---

## 📜 License
MIT License. Free to use and modify.  
