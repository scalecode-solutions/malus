#![cfg(feature = "core-graphics")]

use malus::core_graphics::*;

#[test]
fn color_space_rgb() {
    let cs = CGColorSpace::device_rgb();
    assert_eq!(cs.number_of_components(), 3);
}

#[test]
fn color_space_gray() {
    let cs = CGColorSpace::device_gray();
    assert_eq!(cs.number_of_components(), 1);
}

#[test]
fn color_space_srgb() {
    let cs = CGColorSpace::srgb();
    assert_eq!(cs.number_of_components(), 3);
}

#[test]
fn color_space_display_p3() {
    let cs = CGColorSpace::display_p3();
    assert_eq!(cs.number_of_components(), 3);
}

#[test]
fn color_rgba() {
    let c = CGColor::rgba(1.0, 0.5, 0.0, 0.8);
    let comp = c.components();
    assert_eq!(comp.len(), 4);
    assert!((comp[0] - 1.0).abs() < 1e-10);
    assert!((comp[1] - 0.5).abs() < 1e-10);
    assert!((comp[2] - 0.0).abs() < 1e-10);
    assert!((comp[3] - 0.8).abs() < 1e-10);
}

#[test]
fn color_gray() {
    let c = CGColor::gray(0.5, 1.0);
    assert_eq!(c.number_of_components(), 2); // gray + alpha
}

#[test]
fn color_presets() {
    let w = CGColor::white();
    let b = CGColor::black();
    let cl = CGColor::clear();
    assert!((w.alpha() - 1.0).abs() < 1e-10);
    assert!((b.alpha() - 1.0).abs() < 1e-10);
    assert!((cl.alpha() - 0.0).abs() < 1e-10);
}

#[test]
fn bitmap_context_create() {
    let ctx = CGContext::bitmap_rgba(64, 64);
    let img = ctx.create_image();
    assert_eq!(img.width(), 64);
    assert_eq!(img.height(), 64);
}

#[test]
fn bitmap_context_draw() {
    let ctx = CGContext::bitmap_rgba(10, 10);
    let red = CGColor::rgb(1.0, 0.0, 0.0);
    ctx.set_fill_color(&red);
    ctx.fill_rect(CGRect::new(0.0, 0.0, 10.0, 10.0));
    let img = ctx.create_image();
    assert_eq!(img.width(), 10);
    assert_eq!(img.bits_per_component(), 8);
    assert_eq!(img.bits_per_pixel(), 32);
}

#[test]
fn bitmap_context_state() {
    let ctx = CGContext::bitmap_rgba(10, 10);
    ctx.save();
    ctx.translate(5.0, 5.0);
    ctx.restore();
    // Shouldn't crash
}

#[test]
fn image_from_rgba() {
    let data = vec![255u8; 4 * 4 * 4]; // 4x4 white
    let img = CGImage::from_rgba(4, 4, &data);
    assert_eq!(img.width(), 4);
    assert_eq!(img.height(), 4);
    assert_eq!(img.bits_per_component(), 8);
}

#[test]
fn data_provider_from_bytes() {
    let bytes = vec![0u8; 100];
    let _provider = CGDataProvider::from_bytes(&bytes);
    // Just verifying no crash on create + drop
}

#[test]
fn mutable_path() {
    let path = CGMutablePath::new();
    assert!(path.is_empty());

    path.move_to(0.0, 0.0);
    path.line_to(10.0, 0.0);
    path.line_to(10.0, 10.0);
    path.close();
    assert!(!path.is_empty());

    let bbox = path.bounding_box();
    assert!((bbox.size.width - 10.0).abs() < 1e-10);
    assert!((bbox.size.height - 10.0).abs() < 1e-10);
}

#[test]
fn path_freeze() {
    let mp = CGMutablePath::new();
    mp.move_to(0.0, 0.0);
    mp.line_to(5.0, 5.0);
    let path = mp.to_path();
    assert!(!path.is_empty());
}

#[test]
fn display_main() {
    let d = CGDisplay::main();
    assert!(d.pixels_wide() > 0);
    assert!(d.pixels_high() > 0);
}

#[test]
fn display_list() {
    let displays = CGDisplay::active_displays();
    assert!(!displays.is_empty());
    assert!(displays.contains(&CGDisplay::main()));
}

#[test]
fn affine_transform_identity() {
    let t = CGAffineTransform::IDENTITY;
    assert!(t.is_identity());
}

#[test]
fn affine_transform_translate() {
    let t = CGAffineTransform::translation(10.0, 20.0);
    let p = t.apply_to_point(CGPoint { x: 0.0, y: 0.0 });
    assert!((p.x - 10.0).abs() < 1e-10);
    assert!((p.y - 20.0).abs() < 1e-10);
}

#[test]
fn affine_transform_concat() {
    let t1 = CGAffineTransform::translation(5.0, 0.0);
    let t2 = CGAffineTransform::translation(0.0, 5.0);
    let combined = t1.concat(t2);
    let p = combined.apply_to_point(CGPoint { x: 0.0, y: 0.0 });
    assert!((p.x - 5.0).abs() < 1e-10);
    assert!((p.y - 5.0).abs() < 1e-10);
}

#[test]
fn gradient_create() {
    let g = CGGradient::two_color([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]);
    // Just verifying no crash on create + drop
    let _ = g;
}

#[test]
fn gradient_draw_linear() {
    let ctx = CGContext::bitmap_rgba(10, 10);
    let g = CGGradient::two_color([1.0, 0.0, 0.0, 1.0], [0.0, 0.0, 1.0, 1.0]);
    g.draw_linear(
        &ctx,
        CGPoint { x: 0.0, y: 0.0 },
        CGPoint { x: 10.0, y: 10.0 },
        CGGradientDrawingOptions::Both,
    );
    let _img = ctx.create_image();
}
