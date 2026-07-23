#[cfg(test)]
mod translation_tests {
    use crate::current_lang;
    use crate::set_lang;
    use crate::tr;

    #[test]
    fn zh_cn_translations_work() {
        let original = current_lang();

        set_lang("zh-CN");
        assert_eq!(current_lang(), "zh-CN");

        // Verify actual translations work
        assert_eq!(tr!("Switch language"), "切换语言");
        assert_eq!(tr!("Quit the application"), "退出应用程序");
        assert_eq!(tr!("Start a new session"), "开始新会话");
        assert_eq!(tr!("Open the settings modal"), "打开设置面板");

        // Verify English fallback for untranslated keys
        assert_eq!(tr!("Some unknown key"), "Some unknown key");

        // Reset to original
        set_lang(&original);
    }

    #[test]
    fn ja_translations_work() {
        let original = current_lang();

        set_lang("ja");
        assert_eq!(current_lang(), "ja");
        assert_eq!(tr!("Switch language"), "言語を切り替え");
        assert_eq!(tr!("Quit the application"), "アプリケーションを終了");

        set_lang(&original);
    }

    #[test]
    fn round_trip_en_to_zh_and_back() {
        let original = current_lang();

        set_lang("zh-CN");
        let zh = tr!("Switch language");
        assert_eq!(zh, "切换语言");

        set_lang("en");
        let en = tr!("Switch language");
        assert_eq!(en, "Switch language");

        set_lang(&original);
    }
}
