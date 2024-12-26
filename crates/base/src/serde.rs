pub mod datetime {
    use chrono::{DateTime, FixedOffset, NaiveDateTime, TimeDelta};
    use serde::{self, de::Error, Deserialize, Deserializer, Serializer};

    const ASIA_SHANGHAI_OFFSET: TimeDelta = TimeDelta::hours(8);
    const ASIA_SHANGHAI: FixedOffset =
        FixedOffset::east_opt(ASIA_SHANGHAI_OFFSET.num_seconds() as _).unwrap();
    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(date: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.format(FORMAT).to_string())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
            .and_then(|s| NaiveDateTime::parse_from_str(&s, FORMAT).map_err(Error::custom))
            .map(|datetime| DateTime::from_naive_utc_and_offset(datetime, ASIA_SHANGHAI))
            .map(|datetime| datetime - ASIA_SHANGHAI_OFFSET)
    }
}

#[derive(Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl std::str::FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s
            .strip_prefix('#')
            .ok_or_else(|| format!("Color {s:?} not leading with #"))?;
        if s.len() != 6 && s.len() != 8 {
            return Err(format!("Color {s:?} length not 6 or 8"));
        }
        let mut color = Color(
            u8::from_str_radix(&s[0..2], 16)
                .map_err(|_| format!("Color {:?} not valid hex number", &s[0..2]))?,
            u8::from_str_radix(&s[2..4], 16)
                .map_err(|_| format!("Color {:?} not valid hex number", &s[2..4]))?,
            u8::from_str_radix(&s[4..6], 16)
                .map_err(|_| format!("Color {:?} not valid hex number", &s[4..6]))?,
            0,
        );
        color.3 = if s.len() == 8 {
            u8::from_str_radix(&s[6..8], 16)
                .map_err(|_| format!("Color {:?} not valid hex color", &s[6..8]))?
        } else {
            0xffu8
        };
        Ok(color)
    }
}

impl<'de> serde::Deserialize<'de> for Color {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <Self as std::str::FromStr>::from_str(&String::deserialize(deserializer)?)
            .map_err(serde::de::Error::custom)
    }
}

impl serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2))?;
        if self.3 != 0 {
            f.write_fmt(format_args!("{:02x}", self.3))?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_fmt(format_args!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2))?;
        if self.3 != 0 {
            f.write_fmt(format_args!("{:02x}", self.3))?;
        }
        Ok(())
    }
}
