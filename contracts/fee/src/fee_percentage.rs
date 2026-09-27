/// Converts a fee expressed in basis points into a percentage string.
///
/// The fractional part is always rendered with two digits (for example,
/// `150` basis points becomes `"1.50%"`).
///
/// # Arguments
///
/// * `bps` - The fee in basis points, where 100 basis points equals 1 percent.
pub fn fee_bps_to_display(bps: u32) -> String {
    let whole = bps / 100;
    let frac = bps % 100;
    format!("{}.{:02}%", whole, frac)
}

/// Returns the fee percentage in the contract's basis-point representation.
///
/// # Arguments
///
/// * `bps` - The fee in basis points, where 100 basis points equals 1 percent.
///
/// # Returns
///
/// The unchanged basis-point value.
pub fn get_fee_percentage(bps: u32) -> u32 {
    bps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_1_percent() {
        assert_eq!(fee_bps_to_display(100), "1.00%");
    }

    #[test]
    fn test_display_half_percent() {
        assert_eq!(fee_bps_to_display(50), "0.50%");
    }

    #[test]
    fn test_get_fee_percentage() {
        assert_eq!(get_fee_percentage(250), 250);
    }
}
