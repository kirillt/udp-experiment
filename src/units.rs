use std::str::FromStr;

pub const MILLISECONDS: &'static str = "ms";
pub const MICROSECONDS: &'static str = "mcs";
pub const NANOSECONDS: &'static str = "ns";
pub const SECONDS: &'static str = "s";
pub const MINUTES: &'static str = "m";
pub const HOURS: &'static str = "h";
pub const DAYS: &'static str = "d";
//What if you want to use it as health-check?

pub const KILO: &'static str = "K";
pub const MEGA: &'static str = "M";
pub const GIGA: &'static str = "G";
pub const TERA: &'static str = "T";
pub const PETA: &'static str = "P";
pub const EXA:  &'static str = "E";
//ZETTA is too much for Rust :)

pub fn parse_number_with_suffix(text: &str) -> Result<(usize, String), &'static str> {
    let mut chars = text.chars();

    let mut curr = chars.next();
    let mut buffer = vec![];
    let mut suffix = vec![];
    while curr.is_some() {
        let c = curr.unwrap();
        if c.is_digit(10) {
            buffer.push(c);
            curr = chars.next();
        } else {
            suffix.push(c);
            break;
        }
    }
    if buffer.is_empty() {
        return Err("Number contains no digits");
    }
    let amount: String = buffer.into_iter().collect();

    for c in chars {
        suffix.push(c);
    }
    let suffix: String = suffix.into_iter().collect();
    if let Ok(amount) = usize::from_str(&amount) {
        if amount <= 0 {
            return Err("Only strictly positive numbers, please!");
        }
        return Ok((amount, suffix));
    }
    return Err("Number has too many digits");
}

#[allow(dead_code)]
pub fn parse_amount(text: &str) -> Result<usize, &'static str> {
    let (amount, suffix) = parse_number_with_suffix(text)?;

    if suffix == "" {
        return Ok(amount);
    }
    if suffix == KILO {
        return Ok(1_000 * amount);
    }
    if suffix == MEGA {
        return Ok(1_000_000 * amount);
    }
    if suffix == GIGA {
        return Ok(1_000_000_000 * amount);
    }
    if suffix == TERA {
        return Ok(1_000_000_000_000 * amount);
    }
    if suffix == PETA {
        return Ok(1_000_000_000_000_000 * amount);
    }
    if suffix == EXA {
        return Ok(1_000_000_000_000_000_000 * amount);
    }
    return Err("Amount value contains unknown suffix");
}

pub fn parse_duration(text: &str) -> Result<usize, &'static str> {
    let (amount, suffix) = parse_number_with_suffix(text)?;

    if suffix == DAYS || suffix == ""{
        return Ok(86_400_000_000_000 * amount);
    }
    if suffix == HOURS || suffix == ""{
        return Ok(3_600_000_000_000 * amount);
    }
    if suffix == MINUTES || suffix == ""{
        return Ok(60_000_000_000 * amount);
    }
    if suffix == SECONDS || suffix == ""{
        return Ok(1_000_000_000 * amount);
    }
    if suffix == MILLISECONDS {
        return Ok(1_000_000 * amount);
    }
    if suffix == MICROSECONDS {
        return Ok(1_000 * amount);
    }
    if suffix == NANOSECONDS {
        return Ok(amount);
    }
    return Err("Duration value contains unknown suffix");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn parsing_is_correct() {
        assert_eq!(parse_duration("1"), Ok(1_000_000_000));
        assert_eq!(parse_duration("123"), Ok(123_000_000_000));
        assert_eq!(parse_duration("123"), Ok(123_000_000_000));
        assert_eq!(parse_duration("23ms"), Ok(23_000_000));
        assert_eq!(parse_duration("3mcs"), Ok(3_000));
        assert_eq!(parse_duration("3ns"), Ok(3));
        assert_eq!(parse_duration("0"), Err("Only strictly positive numbers, please!"));
        assert_eq!(parse_duration("00"), Err("Only strictly positive numbers, please!"));
        assert_eq!(parse_duration("ms"), Err("Number contains no digits"));
        assert_eq!(parse_duration("123xx"), Err("Duration value contains unknown suffix"));
        assert_eq!(parse_amount("1"), Ok(1));
        assert_eq!(parse_amount("10"), Ok(10));
        assert_eq!(parse_amount("1K"), Ok(1000));
        assert_eq!(parse_amount("1M"), Ok(1_000_000));
    }
}
