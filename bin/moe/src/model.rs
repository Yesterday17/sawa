use gemini_rust::{Model, ThinkingConfig, ThinkingLevel};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BoundingBox {
    /// [y1, x1, y2, x2] in normalized coordinates (0-1000)
    #[serde(rename = "box_2d")]
    pub box_2d: [u32; 4],
    pub label: Option<String>,
}

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum GeminiModel {
    #[clap(name = "gemini-3.0-flash")]
    Gemini3Flash,
    #[clap(name = "gemini-3.0-pro")]
    Gemini3Pro,
    #[clap(name = "gemini-2.5-flash")]
    Gemini25Flash,
    #[clap(name = "gemini-2.5-pro")]
    Gemini25Pro,
    #[clap(name = "gemini-2.5-flash-lite")]
    Gemini25FlashLite,
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
    #[clap(aliases = ["oshinagaki", "comiket"])]
    Goods,
}

impl PromptPreset {
    pub fn prompt(&self) -> &'static str {
        match self {
            PromptPreset::Photo => {
                r#"Detect the 2d bounding boxes of all the entire Nama Shashin cards.
Crucial: The bounding box must encompass the full physical item, strictly including the text/description area at the bottom and any borders. Do not cut off the bottom text."#
            }
            PromptPreset::Goods => {
                r#"Task: Detect all relevant bounding boxes on the Oshinagaki menu.

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
