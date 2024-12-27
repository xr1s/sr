use super::Formatter;

pub(crate) trait Formattable<T> {
    /// #1% 形式
    fn write_raw(&mut self, value: &T, percent: bool);
    /// #1[i]% 形式
    fn write_int(&mut self, value: &T, percent: bool);
    /// #1[f1]% 形式
    fn write_float(&mut self, value: &T, prec: u32, percent: bool);
}

impl<'a, Data: crate::data::GameData> Formattable<&'a str> for Formatter<'_, Data> {
    fn write_raw(&mut self, value: &&'a str, percent: bool) {
        self.push_str(value);
        if percent {
            self.push('%');
        }
    }

    fn write_int(&mut self, value: &&'a str, percent: bool) {
        self.push_str(value);
        self.push_str("[i]");
        if percent {
            self.push('%');
        }
    }

    fn write_float(&mut self, value: &&'a str, prec: u32, percent: bool) {
        self.push_str(value);
        self.push_str("[f");
        if prec != 0 {
            self.push_str(&prec.to_string());
            self.push(']');
        }
        if percent {
            self.push('%');
        }
    }
}

impl<Data: crate::data::GameData> Formattable<u64> for Formatter<'_, Data> {
    fn write_raw(&mut self, value: &u64, percent: bool) {
        self.write_int(value, percent);
    }

    fn write_int(&mut self, value: &u64, percent: bool) {
        use thousands::Separable;
        let value = if percent { *value * 100 } else { *value };
        self.push_str(&value.separate_with_commas());
        if percent {
            self.push('%');
        }
    }

    fn write_float(&mut self, value: &u64, prec: u32, percent: bool) {
        let value = *value as f64;
        <Self as Formattable<f64>>::write_float(self, &value, prec, percent);
    }
}

impl<Data: crate::data::GameData> Formattable<i64> for Formatter<'_, Data> {
    fn write_raw(&mut self, value: &i64, percent: bool) {
        self.write_int(value, percent);
    }

    fn write_int(&mut self, value: &i64, percent: bool) {
        use thousands::Separable;
        let value = if percent { *value * 100 } else { *value };
        self.push_str(&value.separate_with_commas());
        if percent {
            self.push('%');
        }
    }

    fn write_float(&mut self, value: &i64, prec: u32, percent: bool) {
        let value = *value as f64;
        <Self as Formattable<f64>>::write_float(self, &value, prec, percent);
    }
}

impl<Data: crate::data::GameData> Formattable<f64> for Formatter<'_, Data> {
    fn write_raw(&mut self, value: &f64, percent: bool) {
        let value = f64::round(if percent { *value * 100. } else { *value });
        self.push_str(&value.to_string());
    }

    fn write_int(&mut self, value: &f64, percent: bool) {
        let value = f64::round(if percent { *value * 100. } else { *value }) as u64;
        <Self as Formattable<u64>>::write_int(self, &value, false);
        if percent {
            self.push('%');
        }
    }

    fn write_float(&mut self, value: &f64, prec: u32, percent: bool) {
        let value = if percent { *value * 100. } else { *value };
        let prec_10 = 10usize.pow(prec) as f64;
        let value = f64::round(value * prec_10) / prec_10;
        self.push_str(&value.to_string());
        if percent {
            self.push('%');
        }
    }
}
