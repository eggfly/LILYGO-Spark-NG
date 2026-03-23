use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Language {
    English,
    SimplifiedChinese,
    TraditionalChinese,
    Japanese,
}

impl Language {
    pub const ALL: [Language; 4] = [
        Language::English,
        Language::SimplifiedChinese,
        Language::TraditionalChinese,
        Language::Japanese,
    ];

    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::SimplifiedChinese => "zh-CN",
            Language::TraditionalChinese => "zh-TW",
            Language::Japanese => "ja",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::SimplifiedChinese => "中文简体",
            Language::TraditionalChinese => "中文繁體",
            Language::Japanese => "日本語",
        }
    }

    pub fn from_system() -> Self {
        let locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
        if locale.starts_with("zh") {
            if locale.contains("TW") || locale.contains("Hant") || locale.contains("HK") {
                Language::TraditionalChinese
            } else {
                Language::SimplifiedChinese
            }
        } else if locale.starts_with("ja") {
            Language::Japanese
        } else {
            Language::English
        }
    }
}

pub struct I18n {
    translations: HashMap<&'static str, [&'static str; 4]>,
    pub language: Language,
}

impl I18n {
    pub fn new(language: Language) -> Self {
        let mut t = HashMap::new();

        // [en, zh-CN, zh-TW, ja]
        // Navigation
        t.insert("nav.discovery", ["Discovery", "发现", "發現", "ディスカバリー"]);
        t.insert("nav.firmware", ["Firmware Center", "固件中心", "固件中心", "ファームウェア"]);
        t.insert("nav.firmware_lab", ["Firmware Lab", "固件研究所", "韌體研究所", "ファームウェア研究所"]);
        t.insert("nav.serial_tools", ["Serial Tools", "串口工具", "串口工具", "シリアルツール"]);
        t.insert("nav.embedded_tools", ["Embedded Tools", "哆啦A梦百宝箱", "哆啦A夢百寶箱", "組込みツール"]);
        t.insert("nav.community", ["LILYGO Related", "LILYGO 相关", "LILYGO 相關", "LILYGO 関連"]);
        t.insert("nav.spark_lab", ["Spark Lab", "Spark Lab", "Spark Lab", "Spark Lab"]);
        t.insert("nav.settings", ["Settings", "设置", "設置", "設定"]);
        t.insert("nav.upload", ["Upload", "固件上传", "韌體上傳", "アップロード"]);

        // Sidebar
        t.insert("sidebar.login_github", ["Login with GitHub", "使用 GitHub 登录", "使用 GitHub 登入", "GitHubでログイン"]);
        t.insert("sidebar.logout", ["Log out", "退出登录", "退出登入", "ログアウト"]);
        t.insert("sidebar.made_with", ["Made with 🤖 AI & ❤️ Love", "Made with 🤖 AI & ❤️ Love", "Made with 🤖 AI & ❤️ Love", "Made with 🤖 AI & ❤️ Love"]);

        // Firmware Center
        t.insert("fc.search", ["Search products...", "搜索设备...", "搜尋裝置...", "デバイスを検索..."]);
        t.insert("fc.only_with_firmware", ["Only show with firmware", "仅显示有固件的产品", "僅顯示有固件的產品", "ファームウェアがある製品のみ表示"]);
        t.insert("fc.products", ["products", "个产品", "個產品", "製品"]);
        t.insert("fc.available_firmware", ["Available Firmware", "可用固件", "可用固件", "利用可能なファームウェア"]);
        t.insert("fc.no_firmware", ["No firmware found", "未找到固件", "未找到固件", "ファームウェアが見つかりません"]);
        t.insert("fc.download", ["Download", "下载", "下載", "ダウンロード"]);
        t.insert("fc.select_device", ["Select a product", "选择产品", "選擇產品", "製品を選択"]);

        // Discovery
        t.insert("discovery.title", ["Spark Discovery", "发现", "發現", "ディスカバリー"]);
        t.insert("discovery.subtitle", ["Latest news and inspiration from the open-source embedded community.", "探索嵌入式社区的最新新闻和灵感。", "探索嵌入式社區的最新新聞和靈感。", "オープンソース組込みコミュニティの最新ニュースとインスピレーション。"]);

        // Firmware Lab
        t.insert("lab.burner", ["Burner", "烧录", "燒錄", "書き込み"]);
        t.insert("lab.dumper", ["Dumper", "提取", "提取", "ダンプ"]);
        t.insert("lab.analyzer", ["Analyzer", "分析", "分析", "分析"]);
        t.insert("lab.partition_editor", ["Partition Editor", "分区编辑", "分區編輯", "パーティションエディタ"]);
        t.insert("lab.start_flash", ["Start Flashing", "开始烧录", "開始燒錄", "書き込み開始"]);
        t.insert("lab.ready", ["Ready to flash", "准备就绪", "準備就緒", "書き込み準備完了"]);
        t.insert("lab.drop_file", ["Drop .bin file here or click to select", "拖放 .bin 文件或点击选择", "拖放 .bin 文件或點擊選擇", ".binファイルをドロップまたはクリックして選択"]);

        // Serial Tools
        t.insert("serial.connect", ["Connect", "连接", "連接", "接続"]);
        t.insert("serial.disconnect", ["Disconnect", "断开", "斷開", "切断"]);
        t.insert("serial.clear", ["Clear", "清除", "清除", "クリア"]);
        t.insert("serial.send", ["Send", "发送", "發送", "送信"]);
        t.insert("serial.select_port", ["Select Port", "选择端口", "選擇端口", "ポート選択"]);
        t.insert("serial.type_command", ["Type command...", "输入命令...", "輸入命令...", "コマンドを入力..."]);

        // Embedded Tools
        t.insert("tools.title", ["Embedded Tools", "嵌入式工具", "嵌入式工具", "組込みツール"]);
        t.insert("tools.subtitle", ["Calculators and utilities for embedded development", "嵌入式开发计算器和实用工具", "嵌入式開發計算器和實用工具", "組込み開発用計算機とユーティリティ"]);

        // Community
        t.insert("community.title", ["LILYGO Related", "LILYGO 相关", "LILYGO 相關", "LILYGO 関連"]);
        t.insert("community.subtitle", ["Official resources and community links", "官方资源和社区链接", "官方資源和社區連結", "公式リソースとコミュニティリンク"]);

        // Spark Lab
        t.insert("sparklab.title", ["Spark Lab", "Spark Lab", "Spark Lab", "Spark Lab"]);
        t.insert("sparklab.subtitle", ["Track features and explore what's coming next", "跟踪功能进展和探索未来规划", "追蹤功能進展和探索未來規劃", "機能の進捗を追跡し、今後の予定を探る"]);
        t.insert("sparklab.sparkling_list", ["Sparkling List", "灵感火花", "靈感火花", "インスピレーション"]);
        t.insert("sparklab.guide", ["Guide", "使用指引", "使用指引", "ガイド"]);
        t.insert("sparklab.shipped", ["Shipped", "已发布", "已發布", "リリース済み"]);
        t.insert("sparklab.planned", ["Planned", "计划中", "計劃中", "計画中"]);
        t.insert("sparklab.spark", ["Spark", "灵感", "靈感", "アイデア"]);

        // Settings
        t.insert("settings.title", ["Settings", "设置", "設置", "設定"]);
        t.insert("settings.feedback", ["Feedback", "产品反馈", "產品反饋", "フィードバック"]);
        t.insert("settings.language", ["Language", "语言", "語言", "言語"]);
        t.insert("settings.theme", ["Theme", "主题", "主題", "テーマ"]);
        t.insert("settings.theme_system", ["Follow System", "跟随系统", "跟隨系統", "システムに従う"]);
        t.insert("settings.theme_light", ["Light", "浅色", "淺色", "ライト"]);
        t.insert("settings.theme_dark", ["Dark", "深色", "深色", "ダーク"]);
        t.insert("settings.accent", ["Theme Color", "主题色", "主題色", "テーマカラー"]);
        t.insert("settings.accent_rotating", ["Auto Rotate", "自动轮换", "自動輪換", "自動ローテーション"]);
        t.insert("settings.accent_fixed", ["Fixed Color", "固定颜色", "固定顏色", "固定カラー"]);
        t.insert("settings.accent_rotating_hint", ["Theme color changes every half day, cycling through 8 colors over 4 days", "主题色每半天自动切换一次，8 种颜色 4 天一轮", "主題色每半天自動切換一次，8 種顏色 4 天一輪", "テーマカラーは半日ごとに自動切替、8色を4日で一巡"]);
        t.insert("settings.link_open_mode", ["Link Open Mode", "链接打开方式", "連結開啟方式", "リンクの開き方"]);
        t.insert("settings.link_internal", ["In-App Window", "应用内窗口", "應用內視窗", "アプリ内ウィンドウ"]);
        t.insert("settings.link_external", ["External Browser", "外部浏览器", "外部瀏覽器", "外部ブラウザ"]);
        t.insert("settings.link_hint", ["Product pages, GitHub, and news links open mode.", "产品页、GitHub、新闻等链接将按此设置打开。", "產品頁、GitHub、新聞等連結將依此設定開啟。", "製品ページ、GitHub、ニュースなどのリンクはこの設定で開きます。"]);
        t.insert("settings.glass", ["Glass Effect", "玻璃拟态", "玻璃擬態", "グラス効果"]);
        t.insert("settings.glass_on", ["On", "已开启", "已開啟", "オン"]);
        t.insert("settings.glass_off", ["Off", "已关闭", "已關閉", "オフ"]);
        t.insert("settings.glass_hint", ["Sidebar and settings use frosted glass effect. Turn off on low-end devices.", "侧边栏和设置卡片使用毛玻璃效果，可在低性能设备上关闭。", "側邊欄和設定卡片使用毛玻璃效果，可在低效能裝置上關閉。", "サイドバーと設定カードに毛ガラス効果を適用。低スペックPCではオフに。"]);
        t.insert("settings.sound", ["Flash Success Sound", "烧录完成音效", "燒錄完成音效", "書き込み完了音"]);
        t.insert("settings.sound_on", ["On", "已开启", "已開啟", "オン"]);
        t.insert("settings.sound_off", ["Off", "已关闭", "已關閉", "オフ"]);
        t.insert("settings.sound_hint", ["Play a chime when firmware flashing completes.", "固件烧录完成时播放提示音。", "固件燒錄完成時播放提示音。", "ファームウェア書き込み完了時に音を再生します。"]);
        t.insert("settings.flash_style", ["Flash Success Animation", "烧录成功动画", "燒錄成功動畫", "書き込み成功アニメ"]);
        t.insert("settings.flash_style_hint", ["Celebration animation style when flashing completes.", "烧录完成时显示的庆祝动画样式。", "燒錄完成時顯示的慶祝動畫樣式。", "書き込み完了時の祝賀アニメーション。"]);
        t.insert("settings.easter_eggs", ["Easter Eggs & Effects", "彩蛋与特效", "彩蛋與特效", "イースターエッグ＆エフェクト"]);
        t.insert("settings.easter_konami", ["Enter Konami code anywhere", "任意位置输入 Konami 码", "任意位置輸入 Konami 碼", "どこでも Konami コード入力"]);
        t.insert("settings.easter_flash", ["Flash Success", "烧录成功", "燒錄成功", "書き込み成功"]);
        t.insert("settings.easter_flash_hint", ["Celebration when firmware flashing completes", "固件烧录完成时显示庆祝动画", "固件燒錄完成時顯示慶祝動畫", "ファームウェア書き込み完了時に祝賀アニメーション"]);
        t.insert("settings.easter_device", ["Device Detected", "设备检测", "設備檢測", "デバイス検出"]);
        t.insert("settings.easter_device_hint", ["Shows TARGET ACQUIRED when ESP32 is detected", "检测到 ESP32 时显示 TARGET ACQUIRED", "檢測到 ESP32 時顯示 TARGET ACQUIRED", "ESP32 検出時に TARGET ACQUIRED 表示"]);
        t.insert("settings.check_update", ["Check Update", "检查更新", "檢查更新", "アップデート確認"]);
        t.insert("settings.check_now", ["Check Now", "立即检查", "立即檢查", "今すぐ確認"]);
        t.insert("settings.cache", ["Download Cache", "下载缓存", "下載快取", "ダウンロードキャッシュ"]);
        t.insert("settings.cache_hint", ["Cached firmware downloads in temp directory.", "固件下载缓存存放在临时目录中。", "固件下載快取存放在臨時目錄中。", "一時ディレクトリのファームウェアダウンロードキャッシュ。"]);
        t.insert("settings.cache_empty", ["No cached downloads", "无缓存文件", "無快取檔案", "キャッシュなし"]);
        t.insert("settings.cache_clear", ["Clear All", "清理全部", "清理全部", "すべてクリア"]);
        t.insert("settings.advanced", ["Advanced", "高级模式", "高級模式", "詳細設定"]);
        t.insert("settings.manifest_file", ["Firmware Manifest", "固件清单文件", "固件清單文件", "ファームウェアマニフェスト"]);
        t.insert("settings.manifest_hint", ["Use a local JSON instead of network.", "使用本机 JSON，不再从网络拉取。", "使用本機 JSON，不再從網絡拉取。", "ネットワークではなくローカル JSON を使用。"]);
        t.insert("settings.manifest_select", ["Select File", "选择文件", "選擇文件", "ファイルを選択"]);
        t.insert("settings.manifest_clear", ["Clear", "清除", "清除", "クリア"]);
        t.insert("settings.developer_mode", ["Developer Mode", "开发者模式", "開發者模式", "開発者モード"]);
        t.insert("settings.developer_hint", ["Enable advanced debug features.", "启用高级调试功能。", "啟用進階除錯功能。", "高度なデバッグ機能を有効化。"]);
        t.insert("settings.canary", ["Canary Channel", "Canary 更新频道", "Canary 更新頻道", "カナリアチャンネル"]);
        t.insert("settings.canary_hint", ["Receive beta updates (Pre-release).", "接收测试版更新。", "接收測試版更新。", "ベータアップデート（Pre-release）を受信。"]);

        Self { translations: t, language }
    }

    pub fn t(&self, key: &'static str) -> &'static str {
        let idx = match self.language {
            Language::English => 0,
            Language::SimplifiedChinese => 1,
            Language::TraditionalChinese => 2,
            Language::Japanese => 3,
        };
        self.translations.get(key).map(|arr| arr[idx]).unwrap_or(key)
    }

    pub fn set_language(&mut self, language: Language) {
        self.language = language;
    }
}
