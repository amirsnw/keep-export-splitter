use walkdir::WalkDir;
use scraper::{Html, Selector};
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use std::env;

const MAX_BATCH_SIZE: u64 = 25 * 1024 * 1024; // 25MB

fn find_html_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "html"))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn extract_img_srcs(html_path: &Path) -> Result<Vec<String>> {
    let content = fs::read_to_string(html_path)
        .with_context(|| format!("Failed to read file: {:?}", html_path))?;
    let document = Html::parse_document(&content);
    let selector = Selector::parse("img").unwrap();

    Ok(document.select(&selector)
        .filter_map(|el| el.value().attr("src"))
        .map(|s| s.to_string())
        .collect())
}

fn find_image_in_dir(src_dir: &Path, filename: &str) -> Option<PathBuf> {
    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        if let Some(file_name) = entry.path().file_name().and_then(|n| n.to_str()) {
            if file_name == filename {
                return Some(entry.path().to_path_buf());
            }
        }
    }
    None
}

fn copy_with_size_tracking(
    image_path: &Path,
    target_folder: &Path,
    total_size: &mut u64,
) -> Result<u64> {
    let file_name = image_path.file_name().unwrap();
    let destination = target_folder.join(file_name);

    fs::copy(image_path, &destination)
        .with_context(|| format!("Failed to copy image to {:?}", destination))?;

    let size = fs::metadata(&destination)?.len();
    *total_size += size;

    Ok(size)
}

fn move_html_file(html_path: &Path, target_folder: &Path) -> Result<()> {
    let file_name = html_path.file_name().unwrap();
    let destination = target_folder.join(file_name);
    fs::rename(html_path, &destination)?;
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 || args.contains(&"--help".to_string()) {
        println!("Usage:\n  {} <html_dir> <image_source_dir> <output_dir>", args[0]);
        println!("\nExample:\n  {} \"C:\\Users\\you\\Takeout\\Keep\" \"C:\\Users\\you\\images\" \"C:\\Users\\you\\output\"", args[0]);
        return Ok(());
    }

    let html_dir = Path::new(&args[1]);
    let image_source_dir = Path::new(&args[2]);
	let target_dir = Path::new(&args[3]);

    let mut html_files = find_html_files(html_dir);
    html_files.sort(); // optional, for predictable order

    let base_output = target_dir;
    fs::create_dir_all(&base_output)?;

    let mut batch_index = 0;
    let mut batch_folder = base_output.join(format!("batch-{}", batch_index)).join("Takeout").join("Keep");
    fs::create_dir_all(&batch_folder)?;
    let mut total_size: u64 = 0;

    for html_file in html_files {
        let img_srcs = extract_img_srcs(&html_file)?;
        let mut img_total_for_html = 0;

        for img_src in img_srcs {
            let filename = Path::new(&img_src).file_name().unwrap().to_str().unwrap();
            if let Some(found_image_path) = find_image_in_dir(image_source_dir, filename) {
                let added_size = copy_with_size_tracking(&found_image_path, &batch_folder, &mut total_size)?;
                img_total_for_html += added_size;
            } else {
                println!("Image not found for: {}", filename);
            }
        }

		// Add the size of html
		let html_size = fs::metadata(&html_file)?.len();
		total_size += html_size;

        // Move HTML file to the current batch
        move_html_file(&html_file, &batch_folder)?;

        println!(
            "Processed {:?} ({} bytes). Total batch size: {} bytes",
            html_file.file_name().unwrap(),
            img_total_for_html,
            total_size
        );
		
        // If current batch exceeds size limit, start a new one
        if total_size >= MAX_BATCH_SIZE {
            batch_index += 1;
            batch_folder = base_output.join(format!("batch-{}", batch_index)).join("Takeout").join("Keep");
            fs::create_dir_all(&batch_folder)?;
            total_size = 0;
            println!("💡 Starting new batch: batch_{}", batch_index);
        }
    }

    println!("✅ All HTML files processed.");
    Ok(())
}
