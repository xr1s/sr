use super::Formatter;

pub(crate) trait Formattable {
    /// #1% 形式
    fn write_raw(&self, f: &mut Formatter, percent: bool);
    /// #1[i]% 形式
    fn write_int(&self, f: &mut Formatter, percent: bool);
    /// #1[f1]% 形式
    fn write_float(&self, f: &mut Formatter, prec: u32, percent: bool);
}

impl Formattable for &'_ str {
    fn write_raw(&self, f: &mut Formatter, percent: bool) {
        f.result.push_str(self);
        if percent {
            f.result.push('%');
        }
    }

    fn write_int(&self, f: &mut Formatter, percent: bool) {
        f.result.push_str(self);
        f.result.push_str("[i]");
        if percent {
            f.result.push('%');
        }
    }

    fn write_float(&self, f: &mut Formatter, prec: u32, percent: bool) {
        f.result.push_str(self);
        f.result.push_str("[f");
        if prec != 0 {
            f.result.push_str(&prec.to_string());
            f.result.push(']');
        }
        if percent {
            f.result.push('%');
        }
    }
}

impl Formattable for u64 {
    fn write_raw(&self, f: &mut Formatter, percent: bool) {
        self.write_int(f, percent);
    }

    fn write_int(&self, f: &mut Formatter, percent: bool) {
        use thousands::Separable;
        let value = if percent { *self * 100 } else { *self };
        f.result.push_str(&value.separate_with_commas());
        if percent {
            f.result.push('%');
        }
    }

    fn write_float(&self, f: &mut Formatter, prec: u32, percent: bool) {
        let value = *self as f64;
        Formattable::write_float(&value, f, prec, percent);
    }
}

impl Formattable for i64 {
    fn write_raw(&self, f: &mut Formatter, percent: bool) {
        self.write_int(f, percent);
    }

    fn write_int(&self, f: &mut Formatter, percent: bool) {
        use thousands::Separable;
        let value = if percent { *self * 100 } else { *self };
        f.result.push_str(&value.separate_with_commas());
        if percent {
            f.result.push('%');
        }
    }

    fn write_float(&self, f: &mut Formatter, prec: u32, percent: bool) {
        let value = *self as f64;
        Formattable::write_float(&value, f, prec, percent);
    }
}

impl Formattable for f64 {
    fn write_raw(&self, f: &mut Formatter, percent: bool) {
        let value = f64::round(if percent { *self * 100. } else { *self });
        f.result.push_str(&format!("{}", value));
    }

    fn write_int(&self, f: &mut Formatter, percent: bool) {
        let value = f64::round(if percent { *self * 100. } else { *self }) as u64;
        Formattable::write_int(&value, f, false);
        if percent {
            f.result.push('%');
        }
    }

    fn write_float(&self, f: &mut Formatter, prec: u32, percent: bool) {
        let value = if percent { *self * 100. } else { *self };
        let prec_10 = 10usize.pow(prec) as f64;
        let value = f64::round(value * prec_10) / prec_10;
        f.result.push_str(&format!("{value}"));
        if percent {
            f.result.push('%');
        }
    }
}
