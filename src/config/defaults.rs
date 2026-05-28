use crate::config::{
    AnsiColor, ColorConfig, Config, IconConfig, SegmentConfig, SegmentId, TextStyleConfig,
};
use std::collections::HashMap;

pub fn default_phrases() -> Vec<String> {
    vec![
        "今天的你，比昨天又厉害了一点点 ☀️",
        "世界很大，但这段代码只属于你",
        "喝口水，伸个懒腰，我在帮你跑着呢",
        "别急，好东西值得等一等 🍰",
        "你知道吗，你刚才的想法真的很棒",
        "先笑一个，然后我们继续",
        "窗外的风在吹，代码在跑，一切都刚刚好",
        "偶尔发发呆也没关系的",
        "你已经很棒了，真的 🌟",
        "等我搞定这个，你就可以去摸鱼了",
        "生活不止有代码，还有奶茶和猫 🐱",
        "此刻的你，正在创造一些了不起的东西",
        "放轻松，最坏的结果也不过是 Ctrl+Z",
        "今天的晚霞一定很美，记得抬头看看",
        "你值得一个长长的假期",
        "慢慢来，比较快",
        "距离下班还有... 算了不数了",
        "这段代码写完就奖励自己一块巧克力吧 🍫",
        "宇宙很大，但你的代码能跑就行",
        "你就是自己的太阳 🌞",
        "开心最重要，代码第二",
        "此刻世界上有人在想你，也有人在等这段代码",
        "写完这行，去做点让自己开心的事吧",
        "你今天笑过了吗？现在笑也不晚 😊",
        "一切都会好起来的，包括这段代码",
    ]
    .into_iter()
    .map(String::from)
    .collect()
}

impl Default for Config {
    fn default() -> Self {
        crate::ui::themes::ThemePresets::get_default()
    }
}

pub fn default_segments() -> Vec<SegmentConfig> {
    vec![
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "🤖".to_string(),
                nerd_font: "\u{e26d}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 14 }),
                text: Some(AnsiColor::Color16 { c16: 14 }),
                background: None,
            },
            styles: TextStyleConfig { text_bold: true },
            options: HashMap::new(),
        },
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "📁".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 11 }),
                text: Some(AnsiColor::Color16 { c16: 10 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        },
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "🌿".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 12 }),
                text: Some(AnsiColor::Color16 { c16: 12 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        },
        SegmentConfig {
            id: SegmentId::ContextWindow,
            enabled: true,
            icon: IconConfig {
                plain: "⚡".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 13 }),
                text: Some(AnsiColor::Color16 { c16: 13 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        },
        SegmentConfig {
            id: SegmentId::Session,
            enabled: false,
            icon: IconConfig {
                plain: "📝".to_string(),
                nerd_font: "\u{f19bb}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 2 }),
                text: Some(AnsiColor::Color16 { c16: 2 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        },
        SegmentConfig {
            id: SegmentId::Metrics,
            enabled: false,
            icon: IconConfig {
                plain: "📊".to_string(),
                nerd_font: "\u{f0a9e}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 6 }),
                text: Some(AnsiColor::Color16 { c16: 6 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        },
    ]
}
