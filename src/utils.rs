pub fn parse_money_to_cents(input: &str) -> Option<i64> {
    let normalized = input.trim().replace(',', ".");
    if normalized.is_empty() || normalized.starts_with('-') {
        return None;
    }

    let parts: Vec<&str> = normalized.split('.').collect();
    if parts.len() > 2 {
        return None;
    }

    let reais = parts[0].parse::<i64>().ok()?;
    let cents = match parts.get(1) {
        Some(decimal) => {
            let mut digits = decimal
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            if digits.len() > 2 {
                digits.truncate(2);
            }
            while digits.len() < 2 {
                digits.push('0');
            }
            digits.parse::<i64>().ok()?
        }
        None => 0,
    };

    Some(reais.saturating_mul(100).saturating_add(cents))
}

pub fn parse_quantity_to_milli(input: &str) -> Option<i64> {
    let normalized = input.trim().replace(',', ".");
    if normalized.is_empty() || normalized.starts_with('-') {
        return None;
    }

    let parts: Vec<&str> = normalized.split('.').collect();
    if parts.len() > 2 {
        return None;
    }

    let whole = parts[0].parse::<i64>().ok()?;
    let fraction = match parts.get(1) {
        Some(decimal) => {
            let mut digits = decimal
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            if digits.len() > 3 {
                digits.truncate(3);
            }
            while digits.len() < 3 {
                digits.push('0');
            }
            digits.parse::<i64>().ok()?
        }
        None => 0,
    };

    Some(whole.saturating_mul(1000).saturating_add(fraction))
}

pub fn format_money(cents: i64) -> String {
    let sign = if cents < 0 { "-" } else { "" };
    let abs = cents.abs();
    let units = abs / 100;
    let decimals = abs % 100;

    if decimals == 0 {
        format!("{}$ {}", sign, units)
    } else {
        format!("{}$ {}.{:02}", sign, units, decimals)
    }
}

pub fn format_signed_money(cents: i64) -> String {
    let sign = if cents >= 0 { "+" } else { "-" };
    let abs = cents.abs();
    let units = abs / 100;
    let decimals = abs % 100;

    if decimals == 0 {
        format!("{}{}", sign, units)
    } else {
        format!("{}{}.{:02}", sign, units, decimals)
    }
}

pub fn format_quantity(quantity_milli: i64) -> String {
    let whole = quantity_milli / 1000;
    let fraction = quantity_milli % 1000;

    if fraction == 0 {
        whole.to_string()
    } else if fraction % 100 == 0 {
        format!("{}.{}", whole, fraction / 100)
    } else if fraction % 10 == 0 {
        format!("{}.{:02}", whole, fraction / 10)
    } else {
        format!("{}.{:03}", whole, fraction)
    }
}

pub fn change_cents(
    current_unit_value_cents: i64,
    bought_for_cents: i64,
    quantity_milli: i64,
) -> i64 {
    (current_unit_value_cents - bought_for_cents) * quantity_milli / 1000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_bitcoin_change_from_course_example() {
        let current = 1_000;
        let a = change_cents(current, 1_500, 5_000);
        let b = change_cents(current, 500, 10_000);
        assert_eq!(a + b, 2_500);
    }

    #[test]
    fn parses_money_without_float() {
        assert_eq!(parse_money_to_cents("5.25"), Some(525));
        assert_eq!(parse_money_to_cents("0,75"), Some(75));
    }
}
