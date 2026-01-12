use anyhow::{Context, Result};
use base64::{Engine, engine::general_purpose::STANDARD};
use clap::Parser;
use gemini_rust::{MediaResolutionLevel, prelude::*};
use image::{DynamicImage, GenericImageView, Pixel, Rgba};
use std::path::{Path, PathBuf};

use crate::model::{BoundingBox, GeminiModel, PromptPreset};

mod model;

/// Image splitting CLI that uses Gemini API for object detection
#[derive(Parser, Debug)]
#[command(name = "moe")]
#[command(about = "Split images into sub-images using Gemini API object detection")]
struct Args {
    /// Output directory for split images
    #[arg(short, long, default_value = "./output")]
    output: PathBuf,

    /// Gemini API key
    #[arg(short, long, env = "GEMINI_API_KEY")]
    api_key: String,

    /// Model ID to use
    #[arg(long, value_enum, default_value_t = GeminiModel::Gemini3Flash)]
    model: GeminiModel,

    /// Trim white borders from cropped images
    #[arg(long)]
    trim_white: bool,

    /// White threshold for trimming (0-255, default: 240)
    #[arg(long, default_value = "230")]
    white_threshold: u8,

    /// Maximum number of objects to detect
    #[arg(long, default_value = "100")]
    max_objects: u32,

    #[clap(flatten)]
    prompts: PromptsArgs,

    /// Path to the input image file
    input: PathBuf,
}

#[derive(clap::Args, Debug)]
#[group(multiple = false)]
struct PromptsArgs {
    /// Prompt preset to use
    #[clap(long, value_enum, default_value_t = PromptPreset::Photo)]
    preset: PromptPreset,

    /// Custom prompt for object detection
    #[clap(short, long)]
    prompt: Option<String>,
}

impl PromptsArgs {
    pub fn prompt(&self) -> &str {
        self.prompt.as_deref().unwrap_or(self.preset.prompt())
    }
}

/// Call Gemini API to detect bounding boxes in the image
async fn detect_bounding_boxes(
    gemini: &Gemini,
    model: &GeminiModel,
    image: &DynamicImage,
    prompt: &str,
    max_objects: u32,
) -> Result<Vec<BoundingBox>> {
    let system_instruction = format!(
        r#"Return bounding boxes as a JSON array with labels. Never return masks or code fencing. Limit to {max_objects} objects.
If an object is present multiple times, name them with incrementing numbers. If the object belongs to a set, use set label as prefix."#
    );

    // Convert image to PNG bytes
    let mut image_bytes = Vec::new();
    {
        let mut cursor = std::io::Cursor::new(&mut image_bytes);
        image.write_to(&mut cursor, image::ImageFormat::Png)?;
    }

    // Encode image to base64
    let base64_image = STANDARD.encode(&image_bytes);

    let schema = serde_json::json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "box_2d": {
                    "type": "array",
                    "items": {
                        "type": "number"
                    },
                    "description": "Bounding box coordinates in the format [y1, x1, y2, x2]"
                },
                "label": {
                    "type": "string",
                    "description": "Label of the object"
                }
            },
            "required": ["box_2d", "label"]
        }
    });

    // Call API using builder pattern
    let response = gemini
        .generate_content()
        .with_system_instruction(system_instruction)
        .with_user_message(prompt)
        .with_inline_data_and_resolution(
            base64_image,
            "image/png",
            MediaResolutionLevel::MediaResolutionHigh,
        )
        .with_response_mime_type("application/json")
        .with_response_schema(schema)
        .with_temperature(0.5)
        .with_thinking_config(model.thinking_config())
        .execute()
        .await
        .context("Failed to call Gemini API")?;

    if let Some(usage) = &response.usage_metadata {
        log::debug!("Usage: {:#?}", usage);
    }

    let text = response.text();
    log::debug!("Response: {}", text);

    let bounding_boxes: Vec<BoundingBox> = serde_json::from_str(&text)
        .with_context(|| format!("Failed to parse bounding boxes JSON: {}", text))?;

    Ok(bounding_boxes)
}

/// Check if a pixel is considered white based on threshold
fn is_white_pixel(pixel: &Rgba<u8>, threshold: u8) -> bool {
    // For RGBA images, check RGB channels
    // Consider a pixel white if all RGB channels are above threshold
    pixel[0] >= threshold && pixel[1] >= threshold && pixel[2] >= threshold
}

/// Trim white borders from an image
fn trim_white_borders(image: DynamicImage, threshold: u8) -> DynamicImage {
    let (width, height) = image.dimensions();

    if width == 0 || height == 0 {
        return image.clone();
    }

    // Find top border
    let mut top = 0;
    'top_loop: for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            if !is_white_pixel(&pixel.to_rgba(), threshold) {
                top = y;
                break 'top_loop;
            }
        }
    }

    // Skip bottom border
    let bottom = height;

    // Find left border
    let mut left = 0;
    'left_loop: for x in 0..width {
        for y in top..bottom {
            let pixel = image.get_pixel(x, y);
            if !is_white_pixel(&pixel.to_rgba(), threshold) {
                left = x;
                break 'left_loop;
            }
        }
    }

    // Find right border
    let mut right = width;
    'right_loop: for x in (0..width).rev() {
        for y in top..bottom {
            let pixel = image.get_pixel(x, y);
            if !is_white_pixel(&pixel.to_rgba(), threshold) {
                right = x + 1;
                break 'right_loop;
            }
        }
    }

    // Ensure valid bounds
    if right > left && bottom > top {
        let crop_width = right - left;
        let crop_height = bottom - top;
        image.crop_imm(left, top, crop_width, crop_height)
    } else {
        // If no content found, return original image
        image
    }
}

/// Crop and save images based on bounding boxes
fn crop_and_save_images(
    image: &DynamicImage,
    bounding_boxes: &mut [BoundingBox],
    output_dir: &Path,
    input_filename: &str,
    trim_white: bool,
    white_threshold: u8,
) -> Result<()> {
    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir).with_context(|| {
        format!(
            "Failed to create output directory: {}",
            output_dir.display()
        )
    })?;

    // Sort `bounding_boxes` with [x, y]
    bounding_boxes.sort_by_key(|bbox| (bbox.box_2d[1], bbox.box_2d[0]));

    let (width, height) = image.dimensions();

    for (index, bbox) in bounding_boxes.iter().enumerate() {
        // Convert normalized coordinates (0-1000) to absolute coordinates
        let mut y1 = bbox.box_2d[0] * height / 1000;
        let mut x1 = bbox.box_2d[1] * width / 1000;
        let mut y2 = bbox.box_2d[2] * height / 1000;
        let mut x2 = bbox.box_2d[3] * width / 1000;

        // Ensure coordinates are in correct order
        if x1 > x2 {
            std::mem::swap(&mut x1, &mut x2);
        }
        if y1 > y2 {
            std::mem::swap(&mut y1, &mut y2);
        }

        // Clamp to image bounds
        x1 = x1.min(width);
        x2 = x2.min(width);
        y1 = y1.min(height);
        y2 = y2.min(height);

        // Ensure valid rectangle
        if x2 > x1 && y2 > y1 {
            // Crop the image
            let mut cropped = image.crop_imm(x1, y1, x2 - x1, y2 - y1);

            // Trim white borders if enabled
            if trim_white {
                let original_size = cropped.dimensions();
                cropped = trim_white_borders(cropped, white_threshold);
                let trimmed_size = cropped.dimensions();
                if original_size != trimmed_size {
                    log::debug!(
                        "Trimmed white borders: {}x{} -> {}x{}",
                        original_size.0,
                        original_size.1,
                        trimmed_size.0,
                        trimmed_size.1
                    );
                }
            }

            // Generate output filename
            let base_name = Path::new(input_filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("image");
            let label = bbox
                .label
                .as_ref()
                .map(|label| format!("{}_{label}", index + 1))
                .unwrap_or_else(|| format!("object_{}", index + 1));
            let output_path = output_dir.join(format!("{}_{}.png", base_name, label));

            // Save the cropped image
            cropped.save(&output_path).with_context(|| {
                format!("Failed to save cropped image: {}", output_path.display())
            })?;

            println!(
                "Saved: {} (bbox: [{}, {}, {}, {}])",
                output_path.display(),
                x1,
                y1,
                x2,
                y2
            );
        } else {
            eprintln!(
                "Warning: Skipping invalid bounding box {}: [{}, {}, {}, {}]",
                index, x1, y1, x2, y2
            );
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();

    let args = Args::parse();
    log::debug!("Args: {:#?}", args);

    // Validate input file exists
    if !args.input.exists() {
        anyhow::bail!("Input file does not exist: {}", args.input.display());
    }

    // Load image
    log::debug!("Loading image: {}", args.input.display());
    let image = image::open(&args.input)
        .with_context(|| format!("Failed to load image: {}", args.input.display()))?;

    // Get image dimensions
    let (width, height) = image.dimensions();
    log::debug!("Image size: {}x{}", width, height);

    // Initialize Gemini API client
    log::trace!("Initializing Gemini API client...");
    let gemini = Gemini::with_model(args.api_key, Model::from(&args.model))
        .context("Failed to create Gemini client")?;

    // Detect bounding boxes
    log::info!("Detecting bounding boxes with Gemini API...");
    let mut bounding_boxes = detect_bounding_boxes(
        &gemini,
        &args.model,
        &image,
        args.prompts.prompt(),
        args.max_objects,
    )
    .await?;

    // Log bounding boxes
    log::info!("Found {} bounding boxes", bounding_boxes.len());
    for (i, bbox) in bounding_boxes.iter().enumerate() {
        log::info!(
            "  Box {}: {:?} - {}",
            i + 1,
            bbox.box_2d,
            bbox.label.as_deref().unwrap_or("(no label)")
        );
    }

    let input_filename = args
        .input
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("image");
    log::info!("Cropping and saving images to: {}", args.output.display());
    crop_and_save_images(
        &image,
        &mut bounding_boxes,
        &args.output,
        input_filename,
        args.trim_white,
        args.white_threshold,
    )?;

    log::info!(
        "Done! Split {} images saved to {}",
        bounding_boxes.len(),
        args.output.display()
    );

    Ok(())
}
