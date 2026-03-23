pub mod discovery;
pub mod firmware_center;
pub mod firmware_lab;
pub mod serial_tools;
pub mod embedded_tools;
pub mod community;
pub mod spark_lab;
pub mod settings;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Discovery,
    FirmwareCenter,
    FirmwareLab,
    SerialTools,
    EmbeddedTools,
    Community,
    SparkLab,
    Settings,
}

impl Page {
    pub fn id(&self) -> &'static str {
        match self {
            Page::Discovery => "discovery",
            Page::FirmwareCenter => "firmware",
            Page::FirmwareLab => "tools",
            Page::SerialTools => "serial_tools",
            Page::EmbeddedTools => "offline_tools",
            Page::Community => "community",
            Page::SparkLab => "spark_lab",
            Page::Settings => "settings",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Page::Discovery => "Discovery",
            Page::FirmwareCenter => "Firmware Center",
            Page::FirmwareLab => "Firmware Lab",
            Page::SerialTools => "Serial Tools",
            Page::EmbeddedTools => "Embedded Tools",
            Page::Community => "LILYGO Related",
            Page::SparkLab => "Spark Lab",
            Page::Settings => "Settings",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            Page::Discovery => "🧭",
            Page::FirmwareCenter => "📦",
            Page::FirmwareLab => "🔧",
            Page::SerialTools => "💻",
            Page::EmbeddedTools => "📄",
            Page::Community => "👥",
            Page::SparkLab => "🧪",
            Page::Settings => "⚙️",
        }
    }

    pub const ALL: [Page; 8] = [
        Page::Discovery,
        Page::FirmwareCenter,
        Page::FirmwareLab,
        Page::SerialTools,
        Page::EmbeddedTools,
        Page::Community,
        Page::SparkLab,
        Page::Settings,
    ];
}
