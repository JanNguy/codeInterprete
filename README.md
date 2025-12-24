# Universal Code Executor

## Goals

### ⚠️ *Italicized texts represent future goals, not main objectives*

The goal of this project is to execute (or compile and execute) any compatible code across various programming languages *with modular multiple local APIs*.

## Current Architecture

For the moment, the envisaged architecture includes a "main" orchestrator written in Rust. It will detect the shebang and execute the appropriate process.

## Why?

I have several side projects that require this type of program. Maybe it already exists, but I want to build it myself while learning Rust. This project is a subpart of my other big secret project hehe.

---

## 🚀 **Quick Start**

```bash
# Clone the repository
git clone <repository-url>

# Navigate to project
cd universal-code-executor

# Build the project
cargo build --release
```

## 📋 **Supported Languages**

| Language | Status | Execution Method |
|----------|--------|------------------|
| Python | 🚧 In Progress | Direct interpreter |
| JavaScript/Node | ⏳ Planned | Node runtime |
| Rust | ⏳ Planned | Compile & execute |
| Go | ⏳ Planned | Compile & execute |
| Bash/Shell | 🚧 In Progress  | System shell |

## 🏗️ **Architecture**

```
┌─────────────────┐
│   User Input    │
│  (script file)  │
└────────┬────────┘
         │
┌────────▼────────┐
│   Shebang       │
│   Detector      │
└────────┬────────┘
         │
┌────────▼────────┐
│   Language      │
│   Dispatcher    │
└────────┬────────┘
         │
┌────────▼────────┐
│   Executor      │
│   (Rust Core)   │
└────────┬────────┘
         │
┌────────▼────────┐
│   Result/Output │
└─────────────────┘
```
