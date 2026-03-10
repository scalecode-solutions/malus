use malus::core_foundation::*;

#[test]
fn string_roundtrip() {
    let s = CFString::new("hello world");
    assert_eq!(s.to_string(), "hello world");
    assert_eq!(s.len(), 11);
    assert!(!s.is_empty());
}

#[test]
fn string_empty() {
    let s = CFString::new("");
    assert_eq!(s.to_string(), "");
    assert!(s.is_empty());
}

#[test]
fn string_unicode() {
    let input = "cafe\u{0301}"; // decomposed e-acute
    let s = CFString::new(input);
    // CF preserves the original encoding — roundtrip is exact
    assert_eq!(s.to_string(), input);
}

#[test]
fn string_clone_eq() {
    let a = CFString::new("test");
    let b = a.clone();
    assert_eq!(a, b);
}

#[test]
fn string_from_str() {
    let s: CFString = "from trait".into();
    assert_eq!(s.to_string(), "from trait");
}

#[test]
fn string_display() {
    let s = CFString::new("display me");
    assert_eq!(format!("{s}"), "display me");
}

#[test]
fn number_i32() {
    let n = CFNumber::from_i32(42);
    assert_eq!(n.to_i32(), Some(42));
    assert_eq!(n.to_i64(), Some(42));
    assert_eq!(n.to_f64(), Some(42.0));
}

#[test]
fn number_f64() {
    let n = CFNumber::from_f64(3.14);
    let v = n.to_f64().unwrap();
    assert!((v - 3.14).abs() < 1e-10);
}

#[test]
fn number_from_trait() {
    let n: CFNumber = 99i32.into();
    assert_eq!(n.to_i32(), Some(99));
}

#[test]
fn boolean_true() {
    let b = CFBoolean::from_bool(true);
    assert!(b.value());
}

#[test]
fn boolean_false() {
    let b = CFBoolean::from_bool(false);
    assert!(!b.value());
}

#[test]
fn boolean_into() {
    let b = CFBoolean::from_bool(true);
    let v: bool = b.into();
    assert!(v);
}

#[test]
fn data_roundtrip() {
    let d = CFData::new(b"hello bytes");
    assert_eq!(d.as_bytes(), b"hello bytes");
    assert_eq!(d.len(), 11);
}

#[test]
fn data_empty() {
    let d = CFData::new(b"");
    assert!(d.is_empty());
    assert_eq!(d.as_bytes(), b"");
}

#[test]
fn array_from_strings() {
    let a = CFString::new("alpha");
    let b = CFString::new("beta");
    let arr = CFArray::from_type_refs(&[a.as_type_ref(), b.as_type_ref()]);
    assert_eq!(arr.len(), 2);

    // Verify first element is "alpha"
    let ptr = arr.get(0);
    assert!(!ptr.is_null());
    let recovered = unsafe { CFString::retain(ptr as *const __CFString).unwrap() };
    assert_eq!(recovered.to_string(), "alpha");
}

#[test]
fn array_out_of_bounds() {
    let arr = CFArray::from_type_refs(&[]);
    assert!(arr.is_empty());
    assert!(arr.get(0).is_null());
}

#[test]
fn mutable_array() {
    let mut arr = CFMutableArray::new();
    assert!(arr.is_empty());

    let s = CFString::new("item");
    arr.push(s.as_type_ref());
    assert_eq!(arr.len(), 1);
}

#[test]
fn dictionary_from_pairs() {
    let k = CFString::new("key");
    let v = CFNumber::from_i32(123);
    let dict = CFDictionary::from_pairs(
        &[k.as_type_ref()],
        &[v.as_type_ref()],
    );
    assert_eq!(dict.len(), 1);
    assert!(dict.contains_key(k.as_type_ref()));

    let got = dict.get(k.as_type_ref());
    assert!(!got.is_null());
    let recovered = unsafe { CFNumber::retain(got as *const __CFNumber).unwrap() };
    assert_eq!(recovered.to_i32(), Some(123));
}

#[test]
fn mutable_dictionary() {
    let mut dict = CFMutableDictionary::new();
    assert!(dict.is_empty());

    let k = CFString::new("name");
    let v = CFString::new("value");
    dict.set(k.as_type_ref(), v.as_type_ref());
    assert_eq!(dict.len(), 1);
    assert!(dict.contains_key(k.as_type_ref()));

    dict.remove(k.as_type_ref());
    assert!(dict.is_empty());
}

// Expose opaque types needed for retain() calls in tests
use malus::core_foundation::base::{__CFString, __CFNumber};
