# Keep Export Splitter
A Rust-based utility to help you batch and prepare Google Keep exports for import into Notion — with each batch capped at 25 MB to meet Notion’s import size limit.

## ✨ What it does

1. Export your Google Keep notes via [Google Takeout](https://takeout.google.com/).
2. Extract the `.zip` file, which contains all your notes as `.html` files along with images.
3. Run this tool to:
   - Detect all `.html` files with `<img>` tags.
   - Copy the corresponding image files next to their HTML.
   - Organize them into folders, each up to 25MB in size.
   - So you can zip each batch and import into Notion without exceeding limits.

## 🚀 Getting Started

To get started with the Keep Export Splitter, follow these steps:

1. **Install Rust**: Ensure you have Rust installed on your system. You can download it from [rust-lang.org](https://www.rust-lang.org/).

2. **Clone the Repository**: Clone this repository to your local machine using:
   ```bash
   git clone https://github.com/yourusername/keep-export-splitter.git
   ```

3. **Navigate to the Project Directory**: 
   ```bash
   cd keep-export-splitter
   ```

4. **Build the Project**: Compile the project using Cargo, Rust's package manager:
   ```bash
   cargo build --release
   ```

5. **Run the Tool**: Execute the tool with the required arguments:
   ```bash
   cargo run -- "C:\Path\To\Extracted\Keep" "C:\Path\To\Images" "C:\Path\To\Output"
   ```

## 🧪 Usage:
```bash
  target\debug\keep-export-splitter.exe <html_dir> <image_source_dir> <output_dir>
```

#### Example

```bash
cargo run -- "C:\Path\To\Extracted\Keep" "C:\Path\To\Extracted\Keep" "C:\Path\To\Output"
```

## Output
After running, your output folder will look like this:
   ```bash
      C:\Path\To\Output\
      ├── batch-0\Takeout\Keep
      ├── batch-1\Takeout\Keep
      ...
   ```

## 🛠️ Configuration

- **MAX_BATCH_SIZE**: The maximum size for each batch is set to 25MB by default. You can adjust this in the `main.rs` file if needed.

## 📦 Requirements
- Google Keep Takeout .zip file (already extracted)