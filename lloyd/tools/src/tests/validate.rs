#[cfg(test)]
mod tests {
    use crate::validate::{is_email, is_landline_phone, is_mobile_phone, not_empty, is_id_card, is_chinese_name};

    #[test]
    fn test_not_empty() {
        let result = not_empty("a");
        assert!(result, "not_empty not pass");

        let result = not_empty("");
        assert!(!result, "not_empty not pass");
    }

    #[test]
    fn test_is_email() {
        let result = is_email("1119266371@qq.com");
        assert!(result, "is_email not pass");

        let result = is_email("abc");
        assert!(!result, "is_email not pass");
    }

    #[test]
    fn test_is_mobile_phone() {
        let result = is_mobile_phone("15395101257");
        assert!(result, "is_mobile_phone not pass");

        let result = is_mobile_phone("1232323232");
        assert!(!result, "is_mobile_phone not pass");
    }

    #[test]
    fn test_is_landline_phone() {
        let result = is_landline_phone("05646314211");
        assert!(result, "is_landline_phone not pass");

        let result = is_landline_phone("15395101257");
        assert!(!result, "is_landline_phone not pass");
    }

    #[test]
    fn test_is_id_card() {
        let result = is_id_card("342423198903165144");
        assert!(result, "is_id_card not pass");

        let result = is_id_card("15395101257");
        assert!(!result, "is_id_card not pass");
    }

    #[test]
    fn test_is_chinese_name() {
        let result = is_chinese_name("刘动回");
        assert!(result, "is_chinese_name not pass");

        let result = is_chinese_name("15395101257");
        assert!(!result, "is_chinese_name not pass");
    }
}
