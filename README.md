# duper 🦀

A fast and simple command-line tool for finding and removing duplicate files.

`duper` scans a directory, groups files by size, verifies potential duplicates using **SHA-256**, and lets you remove duplicate files safely with user confirmation.

## ✨ Features

* 🔍 Scan any directory
* 📦 Group files by size before hashing
* 🔐 Verify duplicates using SHA-256
* 🎨 Colored terminal output
* 🗑️ Remove duplicate files with confirmation
* 🖥️ Cross-platform
* ⚡ Written in Rust
* 🆘 Built-in `--help`
* 📌 Built-in `--version`

## 🚀 Usage

### Scan a directory

```bash
duper ~/Downloads
```

Example output:

```text
Duplicates found:

/home/mobin/Downloads/song-copy.wav | 41.2743 MB
/home/mobin/Downloads/song.wav      | 41.2743 MB

Delete these files? [Y/N]:
```

Press `Y` to remove duplicate files.

Press `N` or simply press Enter to keep them.

### Help

```bash
duper --help
```

or:

```bash
duper -h
```

### Version

```bash
duper --version
```

or:

```bash
duper -v
```

## 🔎 How it works

`duper` does not consider files duplicates just because they have the same size.

It uses a two-stage process:

```text
Directory
    ↓
File discovery
    ↓
Group by file size
    ↓
SHA-256 hashing
    ↓
Group by hash
    ↓
Duplicate files
    ↓
User confirmation
    ↓
Delete duplicates
```

Grouping by size first avoids calculating hashes for files that obviously cannot be duplicates.

## 📦 Installation

### Linux

Download the latest Linux binary from the [GitHub Releases](https://github.com/mobin-abdi/duper/releases) page.

Extract it:

```bash
tar -xzf duper-linux-x86_64.tar.gz
```

Then run:

```bash
./duper
```

### Windows

Download the Windows `.zip` file from the [GitHub Releases](https://github.com/mobin-abdi/duper/releases) page.

Extract the archive and run:

```text
duper.exe
```

### macOS

Download the appropriate archive for your Mac from the [GitHub Releases](https://github.com/mobin-abdi/duper/releases) page.

Available builds:

* macOS x86_64
* macOS ARM64

## 🛠️ Build from source

Make sure you have Rust installed.

Clone the repository:

```bash
git clone https://github.com/mobin-abdi/duper.git
cd duper
```

Build:

```bash
cargo build --release
```

The binary will be available at:

```text
target/release/duper
```

## 🧪 Development

Run the project with Cargo:

```bash
cargo run -- ~/Downloads
```

Run tests:

```bash
cargo test
```

Check the project:

```bash
cargo check
```

## ⚠️ Important

`duper` can permanently delete files.

Although duplicate files are verified using SHA-256, always review the files shown by the program before confirming deletion.

By default, pressing Enter at the deletion prompt does **not** delete anything.

## 🗺️ Roadmap

* [x] Directory scanning
* [x] File metadata collection
* [x] Size-based candidate detection
* [x] SHA-256 duplicate verification
* [x] Duplicate removal
* [x] Colored output
* [x] CLI arguments
* [x] `--help`
* [x] `--version`
* [x] Cross-platform releases
* [ ] `--dry-run`
* [ ] Better error handling
* [ ] Recursive scanning options
* [ ] Interactive duplicate selection
* [ ] `.deb` packages
* [ ] More package formats

## 📄 License

This project is licensed under the **MIT License**.

See [LICENSE](LICENSE) for the full license text.

## 🤝 Contributing

Contributions, bug reports, and improvements are welcome.

If you find a bug or have an idea for improving `duper`, feel free to open an issue or submit a pull request.

---

Made with 🦀 and Rust.
