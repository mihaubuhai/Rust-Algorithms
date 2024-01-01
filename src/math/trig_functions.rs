/// Function that contains the similarities of the sine and cosine implementations
///
/// Both of them are calculated using their MacLaurin Series
///
/// Because there is just a '+1' that differs in their formula, this function has been
/// created for not repeating
fn template<T: Into<f64>>(x: T, tol: f64, kind: i32) -> f64 {
    use std::f64::consts::PI;
    const PERIOD: f64 = 2.0 * PI;
    /* Sometimes, this function is called for a big 'n'(when tol is very small) */
    fn factorial(n: i128) -> i128 {
        (1..=n).product()
    }

    /* Function to round the 'decimal'th decimal of the number 'x' */
    fn round_up_to_decimal(x: f64, decimal: i32) -> f64 {
        let multiplier = 10f64.powi(decimal);
        (x * multiplier).round() / multiplier
    }

    let mut value: f64 = x.into(); //<-- This is the line for which the trait 'Into' is required

    /* Check for invalid arguments */
    if !value.is_finite() || value.is_nan() {
        println!("This function does not accept invalid arguments.");
        return f64::NAN;
    }

    /*
        The argument to sine could be bigger than the sine's PERIOD
        To prevent overflowing, strip the value off relative to the PERIOD
    */
    while value >= PERIOD {
        value -= PERIOD;
    }
    /* For cases when the value is smaller than the -PERIOD (e.g. sin(-3π) <=> sin(-π)) */
    while value <= -PERIOD {
        value += PERIOD;
    }

    let mut rez = 0f64;
    let mut prev_rez = 1f64;
    let mut step: i32 = 0;
    /*
        This while instruction is the MacLaurin Series for sine / cosine
        sin(x) = Σ (-1)^n * x^2n+1 / (2n+1)!, for n >= 0 and x a Real number
        cos(x) = Σ (-1)^n * x^2n / (2n)!, for n >= 0 and x a Real number

        '+1' in sine's formula is replaced with 'kind', which values are:
            -> kind = 0, for cosine
            -> kind = 1, for sine
    */
    while (prev_rez - rez).abs() > tol {
        prev_rez = rez;
        rez += (-1f64).powi(step) * value.powi(2 * step + kind)
            / factorial((2 * step + kind) as i128) as f64;
        step += 1;
    }

    /* Round up to the 5th decimal */
    round_up_to_decimal(rez, 6)
}

/// Sine function for non radian angle
///
/// Interprets the argument in degrees, not in radians
///
/// ### Example
///
/// sin(1<sup>o</sup>) != \[ sin(1 rad) == sin(π/180) \]
pub fn sine_no_radian_arg<T: Into<f64>>(x: T, tol: f64) -> f64 {
    use std::f64::consts::PI;
    let val: f64 = x.into();
    sine(val * PI / 180f64, tol)
}

/// Returns the value of sin(x), approximated with the given tolerance
///
/// This function supposes the argument is in radians
///
/// ### Example
///
/// sin(1) == sin(1 rad) == sin(π/180)
pub fn sine<T: Into<f64>>(x: T, tol: f64) -> f64 {
    template(x, tol, 1)
}

/// Returns the value of cos, approximated with the given tolerance, for
/// an angle 'x' in radians
pub fn cosine<T: Into<f64>>(x: T, tol: f64) -> f64 {
    template(x, tol, 0)
}

/// Cosine of 'x' in degrees, with the given tolerance
pub fn cosine_no_radian_arg<T: Into<f64>>(x: T, tol: f64) -> f64 {
    use std::f64::consts::PI;
    let val: f64 = x.into();
    cosine(val * PI / 180., tol)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    fn assert<T: Into<f64>>(angle: T, expected_result: f64, is_radian: bool) {
        // I will round the result to 3 decimal places, since it's an approximation.
        match is_radian {
            true => assert_eq!(
                format!("{:.5}", sine(angle, 1e-10)),
                /* Lower the tolerance, the more accurate the value will be */
                format!("{:.5}", expected_result)
            ),
            false => assert_eq!(
                format!("{:.5}", sine_no_radian_arg(angle, 1e-10)),
                format!("{:.5}", expected_result)
            ),
        }
    }

    #[test]
    fn test_sine() {
        assert(0.0, 0.0, true);
        assert(PI / 2.0, 1.0, true);
        assert(PI / 4.0, 1.0 / f64::sqrt(2.0), true);
        assert(PI, -0.0, true);
        assert(PI * 3.0 / 2.0, -1.0, true);
        assert(PI * 2.0, 0.0, true);
        assert(PI * 2.0 * 3.0, 0.0, true);
        assert(-PI, 0.0, true);
        assert(-PI / 2.0, -1.0, true);
        assert(PI * 8.0 / 45.0, 0.5299192642, true);
        assert(0.5, 0.4794255386, true);
        /* Same tests, but angle is now in degrees */
        assert(0, 0.0, false);
        assert(90, 1.0, false);
        assert(45, 1.0 / f64::sqrt(2.0), false);
        assert(180, -0.0, false);
        assert(180 * 3 / 2, -1.0, false);
        assert(180 * 2, 0.0, false);
        assert(180 * 2 * 3, 0.0, false);
        assert(-180, 0.0, false);
        assert(-180 / 2, -1.0, false);
        assert(180 * 8 / 45, 0.5299192642, false);
        assert(0.5, 0.00872654, false);
    }

    #[test]
    fn test_sine_bad_arg() {
        assert!(sine(f64::NEG_INFINITY, 1e-1).is_nan());
        assert!(sine_no_radian_arg(f64::NAN, 1e-1).is_nan());
    }

    #[test]
    fn test_cosine_bad_arg() {
        assert!(cosine(f64::INFINITY, 1e-1).is_nan());
        assert!(cosine_no_radian_arg(f64::NAN, 1e-1).is_nan());
    }

    fn verify<T: Into<f64>>(angle: T, expected_result: f64, is_radian: bool) {
        // I will round the result to 3 decimal places, since it's an approximation.
        match is_radian {
            true => assert_eq!(
                format!("{:.5}", cosine(angle, 1e-10)),
                /* Lower the tolerance, the more accurate the value will be */
                format!("{:.5}", expected_result)
            ),
            false => assert_eq!(
                format!("{:.5}", cosine_no_radian_arg(angle, 1e-10)),
                format!("{:.5}", expected_result)
            ),
        }
    }

    #[test]
    fn test_cosine() {
        use std::f64::consts::PI;
        verify(0, 1., true);
        verify(0, 1., false);
        verify(45, 1. / f64::sqrt(2.), false);
        verify(PI / 4., 1. / f64::sqrt(2.), true);
        verify(90, 0.0, false);
        verify(PI / 2., 0.0, true);
        verify(360, 1., false);
        verify(2. * PI, 1., true);
        verify(15. * PI / 2., 0.0, true);
        verify(-855, -1. / f64::sqrt(2.), false);
    }
}