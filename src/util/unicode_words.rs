extern crate regex;

// Turns out at the moment rust regex do not support lookaheads 
// This is here for future use why I look for other alternatives

pub struct UnicodeCharClasses {
    pub rs_astral_range: String,
    pub rs_combo_marks_range: String,
    pub re_combo_half_marks_range: String,
    pub rs_combo_symbols_range: String,
    pub rs_combo_marks_extended_range: String,
    pub rs_combo_marks_supplement_range: String,
    pub rs_dingbat_range: String,
    pub rs_lower_range: String,
    pub rs_math_op_range: String,
    pub rs_non_char_range: String,
    pub rs_punctuation_range: String,
    pub rs_space_range: String,
    pub rs_upper_range: String,
    pub rs_var_range: String,
    pub rs_apos: String,
    pub rs_break: String,
    pub rs_combo: String,
    pub rs_digit: String,
    pub rs_dingbat: String,
    pub rs_lower: String,
    pub rs_misc: String,
    pub rs_fitz: String,
    pub rs_modifier: String,
    pub rs_non_astral: String,
    pub rs_regional: String,
    pub rs_surr_pair: String,
    pub rs_upper: String,
    pub rs_zwj: String,
    pub rs_misc_lower: String,
    pub rs_misc_upper: String,
    pub rs_opt_contr_lower: String,
    pub rs_opt_contr_upper: String,
    pub re_opt_mod: String,
    pub rs_opt_var: String,
    pub rs_opt_join: String,
    pub rs_ord_lower: String,
    pub rs_ord_upper: String,
    pub rs_seq: String,
    pub rs_emoji: String,
}

impl UnicodeCharClasses {
    pub fn new() -> UnicodeCharClasses {
        let rs_astral_range = String::from("\\ud800-\\udfff");
        let rs_combo_marks_range = String::from("\\u0300-\\u036f");
        let re_combo_half_marks_range = String::from("\\ufe20-\\ufe2f");
        let rs_combo_symbols_range = String::from("\\u20d0-\\u20ff");
        let rs_combo_marks_extended_range = String::from("\\u1ab0-\\u1aff");
        let rs_combo_marks_supplement_range = String::from("\\u1dc0-\\u1dff");
        let rs_dingbat_range = String::from("\\u2700-\\u27bf");
        let rs_lower_range = String::from("a-z\\xdf-\\xf6\\xf8-\\xff");
        let rs_math_op_range = String::from("\\xac\\xb1\\xd7\\xf7");
        let rs_non_char_range = String::from("\\x00-\\x2f\\x3a-\\x40\\x5b-\\x60\\x7b-\\xbf");
        let rs_punctuation_range = String::from("\\u2000-\\u206f");
        let rs_space_range = String::from(" \\t\\x0b\\f\\xa0\\ufeff\\n\\r\\u2028\\u2029\\u1680\\u180e\\u2000\\u2001\\u2002\\u2003\\u2004\\u2005\\u2006\\u2007\\u2008\\u2009\\u200a\\u202f\\u205f\\u3000");
        let rs_upper_range = String::from("A-Z\\xc0-\\xd6\\xd8-\\xde");
        let rs_var_range = String::from("\\ufe0e\\ufe0f");
        let rs_apos = String::from("['\u{2019}]");
        let rs_digit = String::from("\\d");
        let rs_fitz = String::from("\\ud83c[\\udffb-\\udfff]");
        let rs_regional = String::from("(?:\\ud83c[\\udde6-\\uddff]){2}");
        let rs_surr_pair = String::from("[\\ud800-\\udbff][\\udc00-\\udfff]");
        let rs_zwj = String::from("\\u200d");
        let rs_ord_lower = String::from("\\d*(?:1st|2nd|3rd|(?![123])\\dth)(?=\\b|[A-Z_])");
        let rs_ord_upper = String::from("\\d*(?:1ST|2ND|3RD|(?![123])\\dTH)(?=\\b|[a-z_])");

        let rs_break_range = format!(
            "{}{}{}{}{}",
            rs_combo_marks_range,
            re_combo_half_marks_range,
            rs_combo_symbols_range,
            rs_combo_marks_extended_range,
            rs_combo_marks_supplement_range
        );

        let rs_combo_range = format!(
            "{}{}{}{}{}",
            rs_combo_marks_range,
            re_combo_half_marks_range,
            rs_combo_symbols_range,
            rs_combo_marks_extended_range,
            rs_combo_marks_supplement_range
        );
        let rs_lower = format!("[{rs_lower_range}]");
        let rs_break = format!("[{rs_break_range}]");
        let rs_combo = format!("[{rs_combo_range}]");
        let rs_dingbat = format!("[{rs_dingbat_range}]");
        let rs_misc = format!("[^{rs_astral_range}{rs_break_range}{rs_dingbat_range}{rs_lower_range}{rs_upper_range}]");
        let rs_modifier = format!("?:{rs_combo}|{rs_fitz}");
        let rs_non_astral = format!("[^{rs_astral_range}]");
        let rs_upper = format!("[{rs_upper_range}]");
        let rs_misc_lower = format!("(?:{rs_lower}|{rs_misc})");
        let rs_misc_upper = format!("(?:{rs_upper}|{rs_misc})");
        let rs_opt_contr_lower = format!("(?:{rs_apos}(?:d|ll|m|re|s|t|ve))?");
        let rs_opt_contr_upper = format!("(?:{rs_apos}(?:D|LL|M|RE|S|T|VE))?");
        let re_opt_mod = format!("{rs_modifier}?");
        let rs_opt_var = format!("[{rs_var_range}]?");
        let rs_opt_join = format!(
            "(?:{rs_zwj}(?:{rs_non_astral}|{rs_regional}|{rs_surr_pair}){rs_opt_var}{re_opt_mod})*"
        );
        let rs_seq = format!("{rs_opt_var}{re_opt_mod}{rs_opt_join}");
        let rs_emoji = format!("(?:${rs_dingbat}|{rs_regional}|{rs_surr_pair}){rs_seq}");

        let re_unicode_words = regex::Regex::new(format!("{rs_upper}?{rs_lower}+{rs_opt_contr_lower}(?={rs_break}|{rs_upper}|$)|{rs_misc_upper}+{rs_opt_contr_upper}(?=[{rs_break}|{rs_upper}{rs_misc_lower}$)|{rs_upper}?{rs_misc_lower}+{rs_opt_contr_lower}|{rs_upper}+{rs_opt_contr_upper}|{rs_ord_upper}|{rs_ord_lower}|{rs_digit}+|{rs_emoji}").as_str());

        return UnicodeCharClasses {
            rs_astral_range,
            rs_combo_marks_range,
            re_combo_half_marks_range,
            rs_combo_symbols_range,
            rs_combo_marks_extended_range,
            rs_combo_marks_supplement_range,
            rs_dingbat_range,
            rs_lower_range,
            rs_math_op_range,
            rs_non_char_range,
            rs_punctuation_range,
            rs_space_range,
            rs_upper_range,
            rs_var_range,
            rs_apos,
            rs_break,
            rs_combo,
            rs_digit,
            rs_dingbat,
            rs_lower,
            rs_misc,
            rs_fitz,
            rs_modifier,
            rs_non_astral,
            rs_regional,
            rs_surr_pair,
            rs_upper,
            rs_zwj,
            rs_misc_lower,
            rs_misc_upper,
            rs_opt_contr_lower,
            rs_opt_contr_upper,
            re_opt_mod,
            rs_opt_var,
            rs_opt_join,
            rs_ord_lower,
            rs_ord_upper,
            rs_seq,
            rs_emoji,
        };
    }

    fn re_unicode_words(&self) {
        regex::Regex::new(format!("{0}?{1}+{2}(?={3}|{4}|$)|{5}+{6}(?=[{7}|{8}{9}$)|{10}?{11}+{12}|{13}+{14}|{15}|{16}|{17}+|{18}", self.rs_upper, self.rs_lower, self.rs_opt_contr_lower, self.rs_break, self.rs_upper, self.rs_misc_upper, self.rs_opt_contr_upper, self.rs_break, self.rs_upper, self.rs_misc_lower, self.rs_upper, self.rs_misc_lower, self.rs_opt_contr_lower, self.rs_upper, self.rs_opt_contr_upper, self.rs_ord_upper, self.rs_ord_lower, self.rs_digit, self.rs_emoji).as_str());
    }
}
