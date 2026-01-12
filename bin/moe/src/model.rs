use std::str::FromStr;

use gemini_rust::{Model, ThinkingConfig, ThinkingLevel};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BoundingBox {
    /// [y1, x1, y2, x2] in normalized coordinates (0-1000)
    #[serde(rename = "box_2d")]
    pub box_2d: [u32; 4],
    pub label: Option<String>,
}

#[derive(Debug, Clone)]
pub enum GeminiModel {
    Gemini3Flash,
    Gemini3Pro,
    Gemini25Flash,
    Gemini25Pro,
    Gemini25FlashLite,
}

impl FromStr for GeminiModel {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "gemini-3.0-flash" => GeminiModel::Gemini3Flash,
            "gemini-3.0-pro" => GeminiModel::Gemini3Pro,
            "gemini-2.5-flash" => GeminiModel::Gemini25Flash,
            "gemini-2.5-pro" => GeminiModel::Gemini25Pro,
            "gemini-2.5-flash-lite" => GeminiModel::Gemini25FlashLite,
            _ => anyhow::bail!("Invalid Gemini model: {}", s),
        })
    }
}

impl From<&GeminiModel> for Model {
    fn from(model: &GeminiModel) -> Self {
        match model {
            &GeminiModel::Gemini3Flash => Model::Gemini3Flash,
            &GeminiModel::Gemini3Pro => Model::Gemini3Pro,
            &GeminiModel::Gemini25Flash => Model::Gemini25Flash,
            &GeminiModel::Gemini25Pro => Model::Gemini25Pro,
            &GeminiModel::Gemini25FlashLite => Model::Gemini25FlashLite,
        }
    }
}

impl GeminiModel {
    pub fn thinking_config(&self) -> ThinkingConfig {
        match self {
            &GeminiModel::Gemini3Flash => ThinkingConfig {
                thinking_level: Some(ThinkingLevel::Low),
                thinking_budget: None,
                ..Default::default()
            },
            &GeminiModel::Gemini3Pro => ThinkingConfig {
                thinking_level: Some(ThinkingLevel::High),
                thinking_budget: None,
                ..Default::default()
            },
            &GeminiModel::Gemini25Flash => ThinkingConfig {
                thinking_level: None,
                thinking_budget: Some(0),
                ..Default::default()
            },
            &GeminiModel::Gemini25Pro => ThinkingConfig {
                thinking_level: None,
                thinking_budget: Some(128),
                ..Default::default()
            },
            &GeminiModel::Gemini25FlashLite => ThinkingConfig {
                thinking_level: None,
                thinking_budget: Some(512),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum PromptPreset {
    #[clap(alias = "namashashin")]
    Photo,
    #[clap(alias = "oshinagaki")]
    Comiket,
}

impl PromptPreset {
    pub fn prompt(&self) -> &'static str {
        match self {
            PromptPreset::Photo => {
                r#"Detect the 2d bounding boxes of all the entire Nama Shashin cards.
Crucial: The bounding box must encompass the full physical item, strictly including the text/description area at the bottom and any borders. Do not cut off the bottom text."#
            }
            PromptPreset::Comiket => {
                r#"Task: Detect all relevant bounding boxes on the Oshinagaki menu.
Output: A list of JSON objects. Each object must have `box_2d` [ymin, xmin, ymax, xmax] and a `label_type`.

**CRITICAL INSTRUCTION: MULTI-LEVEL DETECTION**
You are required to detect objects at different logical levels simultaneously. **Do not suppress overlapping boxes.** A large "Set" box can and should contain smaller "Item" boxes inside it.

**Define the 3 Distinct Label Types:**

**1. Label: "master_set" (The Container)**
* **Definition:** A group of items sold together under a single collective price (or a distinct background block).
* **Rule:** Draw one large box covering the **entire group** (all content items + title + main price).
* **Example:** A "New Issue Set" box that contains a book, a badge, and a file.

**2. Label: "item_priced" (The Independent Product)**
* **Definition:** A specific item that has its **OWN independent price tag** or "Free/Take One" marker.
* **Rule:** Draw a tight box around the item + its specific price.
* **Scenario A:** A standalone book (¥500). -> Detect as "item_priced".
* **Scenario B:** A book *inside* a Set that *also* has a separate "Single item: ¥500" tag. -> Detect as "item_priced" (inside the "master_set").

**3. Label: "item_unpriced" (The Visual Component)**
* **Definition:** A visually distinct item (like a badge, file, or variant) shown, which **LACKS** a separate price tag (usually because it's just part of the set).
* **Rule:** Draw a tight box around this specific item's image and text.
* **Scenario:** A badge shown inside the "New Issue Set" box with text "Badge included!", but NO separate price. -> Detect as "item_unpriced".

---

**Visual Logic Examples:**
* **Case 1 (Standard Set):** A set containing a Book and a Badge. The Set is ¥2000. The Book is also sold separately for ¥1000. The Badge is not sold separately.
    * **Output:**
        1.  `master_set`: Box around EVERYTHING (Book + Badge + Set Title + ¥2000).
        2.  `item_priced`: Box around the Book (because it has the ¥1000 tag).
        3.  `item_unpriced`: Box around the Badge (visually distinct, but no price tag).

* **Case 2 (Orphaned Items):** A Shikishi displayed with "Will bring" text but no price yet.
    * **Output:**
        1.  `item_unpriced`: Box around the Shikishi (based on intention text).

**Constraint:**
* Strictly distinguish between `item_priced` and `item_unpriced` based on the presence of a numeric price or "Free" marker."#
            }
        }
    }
}
