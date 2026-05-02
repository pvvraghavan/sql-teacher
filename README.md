# 🦀 SQL Teacher — A Rust CLI Learning Application

A terminal-based SQL teaching application built in Rust. It presents structured SQL lessons, tests knowledge with quizzes, gates progression until scores are high enough, and persists progress to disk so learners can resume any time.

---

## 📦 What You Get

This is a **Rust source code project** — it compiles into a **single native executable binary** for your operating system. No runtime, no interpreter, no dependencies needed after compilation.

| OS | Output Binary |
|---------|-------------------------------|
| Windows | `sql_teacher.exe` |
| macOS | `sql_teacher` |
| Linux | `sql_teacher` |

---

## 🛠️ Prerequisites

Your friend needs **Rust** installed (one-time setup, takes ~2 minutes):

### Install Rust

**macOS / Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Windows:**
Download and run the installer from: https://rustup.rs

After installing, restart the terminal and verify:
```bash
rustc --version
cargo --version
```

---

## 🚀 Build & Run

### Option A: Run directly (development mode)
```bash
cd sql_teacher
cargo run
```

### Option B: Build a release binary (faster, optimized)
```bash
cd sql_teacher
cargo build --release
```

The compiled binary will be at:
- **Linux/macOS:** `target/release/sql_teacher`
- **Windows:** `target/release/sql_teacher.exe`

### Option C: Give the binary to your friend (no Rust needed on their machine)

1. Build the release binary on YOUR machine (see Option B above).
2. Copy the binary + the `data/` folder to your friend.
3. Place them side by side:
```
some_folder/
├── sql_teacher          (or sql_teacher.exe on Windows)
└── data/
    ├── modules/
    │   ├── 01.toml
    │   ├── 02.toml
    │   └── ... (all .toml files)
    └── (progress.json will be auto-created here)
```
4. Your friend double-clicks the binary or runs it from terminal:
```bash
./sql_teacher
```

> **Note:** The binary is platform-specific. A binary built on macOS won't run on Windows. Build on the same OS your friend uses, OR have them install Rust and build it themselves.

---

## 📁 Project Structure

```
sql_teacher/
├── Cargo.toml                  ← Dependencies & project metadata
├── README.md                   ← This file
├── data/
│   ├── modules/                ← One .toml file per lesson
│   │   ├── 01.toml             ← SELECT Basics
│   │   ├── 02.toml             ← WHERE Clauses
│   │   ├── 03.toml             ← JOINs
│   │   ├── 04.toml             ← GROUP BY & Aggregates
│   │   ├── 05_major.toml       ← ⭐ Major Quiz #1 (Modules 01-04)
│   │   ├── 06.toml             ← Subqueries
│   │   ├── 07.toml             ← Indexes & Performance
│   │   ├── 08.toml             ← Transactions
│   │   ├── 09.toml             ← Window Functions
│   │   └── 10_major.toml       ← ⭐ Major Quiz #2 (Modules 06-09)
│   └── progress.json           ← Auto-created on first run
└── src/
    ├── main.rs                 ← Entry point & main app loop
    ├── models.rs               ← All data types (structs & enums)
    ├── curriculum.rs           ← Loads & manages module files
    ├── quiz.rs                 ← Quiz engine & score calculation
    ├── progress.rs             ← Save/load user state to JSON
    ├── gate.rs                 ← Gating & progression logic
    └── ui.rs                   ← All terminal rendering
```

---

## 🎓 How It Works

### Curriculum Flow
```
Module 01 → Quiz (95%) → Module 02 → Quiz (95%) → Module 03 → Quiz (95%)
→ Module 04 → Quiz (95%) → ⭐ Major Quiz #1 (90%)
→ Module 06 → Quiz (95%) → Module 07 → Quiz (95%) → Module 08 → Quiz (95%)
→ Module 09 → Quiz (95%) → ⭐ Major Quiz #2 (90%)
→ 🎓 CURRICULUM COMPLETE!
```

### Rules
- **Module Quizzes:** Taken after each lesson. Must score **95%+** to proceed.
- **Major Quizzes:** Taken after every 5th module. Must score **90%+** to proceed.
- **Progress:** Saved to `data/progress.json` automatically after every quiz.
- **Resumable:** Close the app and reopen — progress is restored.

### Features
- Interactive terminal menus (arrow keys to navigate)
- Colored output with pass/fail indicators
- Progress bar showing curriculum completion
- Module selector to review any unlocked module
- Reset option to start fresh

---

## 🧪 Running Tests

The project includes unit tests for the gating logic:

```bash
cargo test
```

Expected output:
```
running 9 tests
test gate::tests::test_first_module_always_unlocked ... ok
test gate::tests::test_second_module_locked_without_first ... ok
test gate::tests::test_second_module_unlocked_after_first ... ok
test gate::tests::test_major_quiz_gates_next_section ... ok
test gate::tests::test_major_quiz_passed_unlocks_next ... ok
test gate::tests::test_next_module_id_new_user ... ok
test gate::tests::test_next_module_after_partial ... ok
test gate::tests::test_curriculum_complete ... ok
test gate::tests::test_progress_fraction ... ok
```

---

## 📚 SQL Modules Covered

| # | Module | Topics |
|---|--------|--------|
| 01 | SELECT Basics | SELECT, FROM, column aliases, SELECT * |
| 02 | WHERE Clauses | Filtering, comparison operators, AND/OR/NOT |
| 03 | JOINs | INNER, LEFT, RIGHT, FULL OUTER JOIN |
| 04 | GROUP BY & Aggregates | COUNT, SUM, AVG, MIN, MAX, HAVING |
| 05 | ⭐ Major Quiz #1 | Covers modules 01–04 |
| 06 | Subqueries | Nested SELECT, correlated, EXISTS |
| 07 | Indexes & Performance | CREATE INDEX, EXPLAIN, query planning |
| 08 | Transactions | BEGIN, COMMIT, ROLLBACK, ACID |
| 09 | Window Functions | ROW_NUMBER, RANK, PARTITION BY, OVER |
| 10 | ⭐ Major Quiz #2 | Covers modules 06–09 |

---

## 🔧 Tech Stack

| Purpose | Crate | Version |
|---------|-------|---------|
| Serialization | serde + serde_json | 1.x |
| TOML Parsing | toml | 0.8 |
| Error Handling | anyhow | 1.x |
| Terminal Colors | colored | 2.x |
| Interactive Menus | inquire | 0.7 |
| Progress Bars | indicatif | 0.17 |

---

Built with 🦀 Rust
