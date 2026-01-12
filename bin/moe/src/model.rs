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
