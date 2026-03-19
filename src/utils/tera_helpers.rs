// src/utils/tera_helpers.rs
use std::collections::HashMap;
use tera::{to_value, try_get_value, Filter, Value};

pub struct LangBarsFilter;

impl Filter for LangBarsFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
        let filled = try_get_value!("lang_bars", "value", u64, value) as u8;
        let bars: Vec<bool> = (1u8..=5).map(|i| i <= filled).collect();
        Ok(to_value(bars)?)
    }
}

pub struct IconClassFilter;

impl Filter for IconClassFilter {
    fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
        let kind = try_get_value!("icon_class", "value", String, value);
        let class = match kind.as_str() {
            "web"   => "cv__experience__item__icon--web",
            "train" => "cv__experience__item__icon--train",
            "film"  => "cv__experience__item__icon--film",
            "post"  => "cv__experience__item__icon--post",
            "drone" => "cv__experience__item__icon--drone",
            _       => "cv__experience__item__icon--default",
        };
        Ok(to_value(class)?)
    }
}
