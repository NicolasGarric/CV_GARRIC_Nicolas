// src/components/sidebar.rs
use crate::data::cv::Language;

pub fn language_bars(lang: &Language) -> Vec<bool> {
    (1usize..=5).map(|i| i <= lang.bars).collect()
}
