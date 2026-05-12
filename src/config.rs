use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CrosshairType {
    Dot,
    Cross,
    T,
    Circle,
}

impl CrosshairType {
    pub fn from_str(s: &str) -> Self {
        match s.trim().to_lowercase().as_str() {
            "dot" => Self::Dot,
            "t" => Self::T,
            "circle" => Self::Circle,
            _ => Self::Cross,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Dot => "dot",
            Self::Cross => "cross",
            Self::T => "t",
            Self::Circle => "circle",
        }
    }
}

#[derive(Clone)]
pub struct Config {
    pub crosshair_type: CrosshairType,
    pub size: f32,
    pub thickness: f32,
    pub color_hex: String,
    pub dot_center: bool,
    pub opacity: f32,
    pub border: bool,
    pub space_width: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            crosshair_type: CrosshairType::Cross,
            size: 24.0,
            thickness: 2.0,
            color_hex: "#FF0000".into(),
            dot_center: true,
            opacity: 0.85,
            border: true,
            space_width: 0.0,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        let config_path = exe_dir.join("config.ini");
        let content = match std::fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(_) => return Self::default(),
        };

        let mut config = Self::default();
        let mut current_section = String::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') {
                current_section = line[1..line.len() - 1].trim().to_lowercase();
                continue;
            }

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_lowercase();
                let value = line[eq_pos + 1..].trim();

                if current_section == "crosshair" {
                    match key.as_str() {
                        "type" => config.crosshair_type = CrosshairType::from_str(value),
                        "size" => config.size = value.parse::<f32>().unwrap_or(24.0).max(4.0),
                        "thickness" => config.thickness = value.parse::<f32>().unwrap_or(2.0).max(1.0),
                        "color" => config.color_hex = value.to_string(),
                        "dot_center" => {
                            config.dot_center = value.eq_ignore_ascii_case("true")
                                || value == "1"
                        }
                        "opacity" => {
                            config.opacity =
                                value.parse::<f32>().unwrap_or(0.85).clamp(0.0, 1.0)
                        }
                        "border" => {
                            config.border = value.eq_ignore_ascii_case("true")
                                || value == "1"
                        }
                        "space_width" => {
                            config.space_width = value.parse::<f32>().unwrap_or(0.0).max(0.0)
                        }
                        _ => {}
                    }
                }
            }
        }

        config
    }

    pub fn parse_color(&self) -> (f32, f32, f32) {
        let hex = self.color_hex.trim_start_matches('#');
        if hex.len() >= 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                return (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
            }
        }
        (1.0, 0.0, 0.0)
    }
}
