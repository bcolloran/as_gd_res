use crate::engine_type_impls::RustCurve;

const CURVE_SAMPLE_POINTS: usize = 64;

fn make_constant_curve(value: f32, min: f32, max: f32) -> RustCurve {
    RustCurve::new_for_test([value; CURVE_SAMPLE_POINTS], value, min, max)
}

fn make_linear_curve(min: f32, max: f32) -> RustCurve {
    let mut baked = [0.0; CURVE_SAMPLE_POINTS];
    let mut integral = 0.0;
    let dx = 1.0 / (CURVE_SAMPLE_POINTS as f32 - 1.0);
    for i in 0..CURVE_SAMPLE_POINTS {
        let t = i as f32 / (CURVE_SAMPLE_POINTS as f32 - 1.0);
        baked[i] = t; // Linear from 0 to 1
        integral += t * dx;
    }
    RustCurve::new_for_test(baked, integral, min, max)
}

#[test]
fn test_try_sample_within_domain() {
    let curve = make_constant_curve(0.5, 0.0, 1.0);
    assert!(curve.try_sample(0.0).is_ok());
    assert!(curve.try_sample(0.5).is_ok());
    assert!(curve.try_sample(1.0).is_ok());
}

#[test]
fn test_try_sample_below_domain() {
    let curve = make_constant_curve(0.5, 0.0, 1.0);
    let result = curve.try_sample(-0.1);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("below the minimum"));
}

#[test]
fn test_try_sample_above_domain() {
    let curve = make_constant_curve(0.5, 0.0, 1.0);
    let result = curve.try_sample(1.1);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("above the maximum"));
}

#[test]
fn test_try_sample_returns_correct_value() {
    let curve = make_constant_curve(0.75, 0.0, 1.0);
    let result = curve.try_sample(0.5);
    assert!(result.is_ok());
    assert!((result.unwrap() - 0.75).abs() < 1e-6);
}

#[test]
fn test_try_sample_at_boundaries() {
    let curve = make_constant_curve(1.0, 0.0, 1.0);
    
    // Sample at exact min boundary
    assert!(curve.try_sample(0.0).is_ok());
    
    // Sample at exact max boundary
    assert!(curve.try_sample(1.0).is_ok());
}

#[test]
fn test_linear_curve_endpoints() {
    let curve = make_linear_curve(0.0, 1.0);
    
    // At min (t=0), should get first baked value (0.0)
    let at_min = curve.try_sample(0.0).unwrap();
    assert!((at_min - 0.0).abs() < 0.02, "at_min = {}", at_min);
    
    // At max (t=1), should get last baked value (1.0)
    let at_max = curve.try_sample(1.0).unwrap();
    assert!((at_max - 1.0).abs() < 0.02, "at_max = {}", at_max);
}

#[test]
fn test_domain_accessors() {
    let curve = make_constant_curve(0.5, -5.0, 10.0);
    assert!((curve.min_domain() - (-5.0)).abs() < 1e-6);
    assert!((curve.max_domain() - 10.0).abs() < 1e-6);
}

#[test]
fn test_integral_accessor() {
    let curve = make_constant_curve(0.5, 0.0, 1.0);
    // Integral of constant function 0.5 over [0,1] is 0.5
    // But our integral is the sum of samples * dx, so let's check it matches
    assert!((curve.integral() - 0.5).abs() < 1e-6);
}

#[test]
fn test_negative_domain() {
    let curve = make_constant_curve(0.5, -10.0, -5.0);
    
    // Within domain
    assert!(curve.try_sample(-7.5).is_ok());
    
    // Outside domain
    assert!(curve.try_sample(-11.0).is_err());
    assert!(curve.try_sample(-4.0).is_err());
}

/// This test verifies that zero-width domains (min == max) are handled correctly.
/// The implementation now returns the first baked value when the domain has zero width.
#[test]
fn test_zero_width_domain_handled_correctly() {
    let curve = make_constant_curve(0.5, 1.0, 1.0); // min == max
    
    // When x == min == max, it should succeed and return the first baked value
    let result = curve.try_sample(1.0);
    assert!(result.is_ok());
    assert!((result.unwrap() - 0.5).abs() < 1e-6);
}
