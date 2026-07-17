/// Parse the key: value lines inside a draw block into a simple config struct.
pub struct DrawConfig {
    pub id: String,
    pub width: u32,
    pub height: u32,
    pub title: String,
    pub background: String,
}

impl DrawConfig {
    #[must_use]
    pub fn from_block(content: &str) -> Self {
        let mut id = "draw-canvas".to_string();
        let mut width = 600u32;
        let mut height = 400u32;
        let mut title = String::new();
        let mut background = "#ffffff".to_string();

        for line in content.lines() {
            // Each line is "key: value"
            if let Some((key, value)) = line.split_once(':') {
                let key = key.trim();
                let value = value.trim();
                match key {
                    "id" => id = value.to_string(),
                    "width" => width = value.parse().unwrap_or(600),
                    "height" => height = value.parse().unwrap_or(400),
                    "title" => title = value.to_string(),
                    "background" => background = value.to_string(),
                    _ => {} // Unknown keys are ignored
                }
            }
        }

        Self {
            id,
            width,
            height,
            title,
            background,
        }
    }
}
