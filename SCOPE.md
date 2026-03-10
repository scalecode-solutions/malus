# Malus — Scope & Architecture

**One-stop Rust bridge to every Apple API. No Swift. No Xcode. No compromises.**

Use the whole thing, or grab one widget. Your call.

---

## Philosophy

- **Have it and not need it** — cover everything Apple exposes, not just what one app needs today. The bindings are mechanical and pattern-based, so breadth is cheap.
- **A la carte** — `use malus::appkit::NSSearchField` grabs just a search field. No framework tax.
- **Layered** — raw FFI at the bottom, typed wrappers in the middle, ergonomic sugar on top. Pick your level.
- **Zero Swift** — pure Rust, talks directly to the ObjC runtime and C APIs.
- **Zero compile-time codegen** — no build scripts parsing headers, no proc macros. Source files are authored with automation but checked in as plain Rust. Fast compiles, full IDE support, nothing magic at build time.

---

## Architecture

```
Layer 3: Declarative UI (optional)
         vstack![ text("Hi"), button("OK").on_click(|| ...) ]
         SwiftUI-like, builds on Layer 2

Layer 2: Ergonomic Wrappers
         Window::new("Title").dark().size(800, 600)
         Typed Rust structs with builder patterns, Drop impls, closures

Layer 1: Framework Bindings
         malus::appkit::NSWindow, malus::metal::MTLDevice, etc.
         Thin typed wrappers around ObjC classes and C functions
         Each class is a newtype around Id with methods

Layer 0: ObjC Runtime
         msg_send!(), sel!(), cls!(), Id, Sel, Class
         Raw runtime access — for when you need to go off-script
         Dynamic class creation, ivar management, protocol conformance
```

Users can mix layers freely. Use Layer 3 for a settings UI, drop to Layer 1 for Metal rendering, hit Layer 0 to register a custom ObjC class.

---

## Module Map

```
malus/
  src/
    lib.rs              — top-level re-exports
    runtime.rs          — Layer 0: ObjC runtime (msg_send!, sel!, cls!, etc.)
    events.rs           — callback/trampoline system

    appkit/             — AppKit framework
      mod.rs
      app.rs            — NSApplication
      window.rs         — NSWindow, NSPanel
      view.rs           — NSView, NSControl
      text_field.rs     — NSTextField
      search_field.rs   — NSSearchField
      text_view.rs      — NSTextView
      button.rs         — NSButton (push, checkbox, radio, switch)
      slider.rs         — NSSlider
      popup_button.rs   — NSPopUpButton
      combo_box.rs      — NSComboBox
      color_well.rs     — NSColorWell
      image_view.rs     — NSImageView
      image.rs          — NSImage, NSBitmapImageRep
      table_view.rs     — NSTableView, NSTableColumn, NSTableCellView
      outline_view.rs   — NSOutlineView (tree views)
      collection_view.rs — NSCollectionView
      scroll_view.rs    — NSScrollView
      split_view.rs     — NSSplitView
      stack_view.rs     — NSStackView
      tab_view.rs       — NSTabView
      box_.rs           — NSBox (separator, group box)
      progress.rs       — NSProgressIndicator
      level.rs          — NSLevelIndicator
      stepper.rs        — NSStepper
      date_picker.rs    — NSDatePicker
      token_field.rs    — NSTokenField
      path_control.rs   — NSPathControl
      browser.rs        — NSBrowser
      menu.rs           — NSMenu, NSMenuItem
      toolbar.rs        — NSToolbar, NSToolbarItem
      touch_bar.rs      — NSTouchBar, NSTouchBarItem
      alert.rs          — NSAlert
      open_panel.rs     — NSOpenPanel
      save_panel.rs     — NSSavePanel
      font_panel.rs     — NSFontPanel, NSFontManager
      color_panel.rs    — NSColorPanel
      sharing.rs        — NSSharingServicePicker, NSSharingService
      pasteboard.rs     — NSPasteboard (clipboard)
      cursor.rs         — NSCursor
      event.rs          — NSEvent
      screen.rs         — NSScreen
      workspace.rs      — NSWorkspace
      notification.rs   — NSUserNotificationCenter / UNUserNotificationCenter
      appearance.rs     — NSAppearance
      visual_effect.rs  — NSVisualEffectView
      responder.rs      — NSResponder chain
      layout.rs         — NSLayoutConstraint, NSLayoutAnchor, Auto Layout helpers
      animation.rs      — NSAnimationContext, NSViewAnimation
      drag_drop.rs      — NSDraggingInfo, drag-and-drop protocols
      accessibility.rs  — NSAccessibility protocols
      printing.rs       — NSPrintOperation, NSPrintInfo
      spell_check.rs    — NSSpellChecker
      speech.rs         — NSSpeechSynthesizer, NSSpeechRecognizer
      user_activity.rs  — NSUserActivity (Handoff, Shortcuts)
      user_defaults.rs  — NSUserDefaults

    metal/              — Metal framework
      mod.rs
      device.rs         — MTLDevice
      command_queue.rs  — MTLCommandQueue
      command_buffer.rs — MTLCommandBuffer
      encoder.rs        — MTLRenderCommandEncoder, MTLComputeCommandEncoder, MTLBlitCommandEncoder
      pipeline.rs       — MTLRenderPipelineState, MTLRenderPipelineDescriptor
      compute.rs        — MTLComputePipelineState, MTLComputePipelineDescriptor
      library.rs        — MTLLibrary, MTLFunction (shader compilation)
      buffer.rs         — MTLBuffer
      texture.rs        — MTLTexture, MTLTextureDescriptor
      sampler.rs        — MTLSamplerState, MTLSamplerDescriptor
      vertex.rs         — MTLVertexDescriptor, MTLVertexAttributeDescriptor
      render_pass.rs    — MTLRenderPassDescriptor, MTLRenderPassColorAttachmentDescriptor
      depth_stencil.rs  — MTLDepthStencilState, MTLDepthStencilDescriptor
      types.rs          — MTLPixelFormat, MTLResourceOptions, MTLPrimitiveType, enums
      capture.rs        — MTLCaptureManager (GPU debugging)
      heap.rs           — MTLHeap (memory management)
      fence.rs          — MTLFence, MTLEvent (synchronization)
      indirect.rs       — MTLIndirectCommandBuffer
      acceleration.rs   — MTLAccelerationStructure (ray tracing)
      layer.rs          — CAMetalLayer, CAMetalDrawable

    core_graphics/      — CoreGraphics framework
      mod.rs
      context.rs        — CGContext (bitmap, PDF, window)
      color_space.rs    — CGColorSpace
      color.rs          — CGColor
      image.rs          — CGImage
      path.rs           — CGPath, CGMutablePath
      gradient.rs       — CGGradient
      pattern.rs        — CGPattern
      font.rs           — CGFont (low-level, prefer CoreText)
      geometry.rs       — CGPoint, CGSize, CGRect, CGAffineTransform
      display.rs        — CGDisplay, CGDirectDisplayID (screen info, capture)
      event.rs          — CGEvent, CGEventTap (global event monitoring)
      window.rs         — CGWindow (window list, screenshots)
      pdf.rs            — CGPDFDocument, CGPDFPage
      data_provider.rs  — CGDataProvider
      data_consumer.rs  — CGDataConsumer
      layer.rs          — CGLayer

    core_text/          — CoreText framework
      mod.rs
      font.rs           — CTFont (create, metrics, glyphs, draw)
      font_descriptor.rs — CTFontDescriptor (attributes, matching)
      font_collection.rs — CTFontCollection (system font enumeration)
      font_manager.rs   — CTFontManager (register/unregister fonts)
      line.rs           — CTLine (text layout)
      run.rs            — CTRun (glyph runs from shaped text)
      frame.rs          — CTFrame, CTFramesetter (multi-line layout)
      paragraph.rs      — CTParagraphStyle
      string_attributes.rs — kCTFontAttributeName, etc.
      types.rs          — CTFontSymbolicTraits, orientation, format enums

    core_foundation/    — CoreFoundation framework
      mod.rs
      base.rs           — CFRetain, CFRelease, CFRef<T> (RAII wrapper)
      string.rs         — CFString <-> Rust String
      number.rs         — CFNumber <-> Rust numeric types
      array.rs          — CFArray, CFMutableArray
      dictionary.rs     — CFDictionary, CFMutableDictionary
      set.rs            — CFSet
      data.rs           — CFData
      url.rs            — CFURL
      date.rs           — CFDate, CFAbsoluteTime
      run_loop.rs       — CFRunLoop, CFRunLoopSource, CFRunLoopTimer
      attributed_string.rs — CFAttributedString, CFMutableAttributedString
      bundle.rs         — CFBundle (app resources)
      preferences.rs    — CFPreferences
      uuid.rs           — CFUUID
      error.rs          — CFError
      boolean.rs        — CFBoolean (kCFBooleanTrue/False)

    quartz_core/        — QuartzCore / Core Animation
      mod.rs
      layer.rs          — CALayer
      metal_layer.rs    — CAMetalLayer, CAMetalDrawable
      animation.rs      — CAAnimation, CABasicAnimation, CAKeyframeAnimation
      transaction.rs    — CATransaction
      display_link.rs   — CADisplayLink
      transform.rs      — CATransform3D

    quick_look/         — QuickLook framework
      mod.rs
      preview_panel.rs  — QLPreviewPanel
      preview_item.rs   — QLPreviewItem protocol
      thumbnail.rs      — QLThumbnailGenerator

    webkit/             — WebKit framework
      mod.rs
      web_view.rs       — WKWebView
      config.rs         — WKWebViewConfiguration
      navigation.rs     — WKNavigationDelegate
      script.rs         — WKUserContentController, WKScriptMessageHandler
      preferences.rs    — WKPreferences

    av_foundation/      — AVFoundation framework
      mod.rs
      audio.rs          — AVAudioPlayer, AVAudioEngine
      speech.rs         — AVSpeechSynthesizer
      capture.rs        — AVCaptureSession, AVCaptureDevice (camera/mic)

    security/           — Security framework
      mod.rs
      keychain.rs       — SecKeychainItem, SecItemAdd/CopyMatching/Delete
      certificate.rs    — SecCertificate
      trust.rs          — SecTrust

    io_kit/             — IOKit framework
      mod.rs
      hid.rs            — IOHIDManager (raw HID device access)
      power.rs          — IOPMAssertionCreateWithName (prevent sleep)
      usb.rs            — IOUSBHostDevice

    network/            — Network.framework
      mod.rs
      connection.rs     — NWConnection
      listener.rs       — NWListener
      endpoint.rs       — NWEndpoint
      parameters.rs     — NWParameters

    uniform_type/       — UniformTypeIdentifiers
      mod.rs
      types.rs          — UTType identifiers

    local_auth/         — LocalAuthentication
      mod.rs
      context.rs        — LAContext (Touch ID / Face ID)

    store_kit/          — StoreKit (in-app purchases, if ever needed)
      mod.rs

    map_kit/            — MapKit
      mod.rs
      map_view.rs       — MKMapView

    dispatch/           — Grand Central Dispatch
      mod.rs
      queue.rs          — dispatch_queue_t, dispatch_async, dispatch_sync
      group.rs          — dispatch_group_t
      semaphore.rs      — dispatch_semaphore_t
      source.rs         — dispatch_source_t (file watchers, timers)

    foundation/         — Foundation extras (beyond CF)
      mod.rs
      process.rs        — NSTask / NSProcess
      file_manager.rs   — NSFileManager
      timer.rs          — NSTimer
      thread.rs         — NSThread
      operation.rs      — NSOperationQueue
      json.rs           — NSJSONSerialization
      regex.rs          — NSRegularExpression
      formatter.rs      — NSDateFormatter, NSNumberFormatter
      locale.rs         — NSLocale
      calendar.rs       — NSCalendar

    ui/                 — Layer 3: Declarative UI (optional)
      mod.rs
      view.rs           — View trait
      text.rs           — text("Hello")
      textfield.rs      — textfield("placeholder")
      button.rs         — button("Click")
      checkbox.rs       — checkbox("Label", checked)
      slider.rs         — slider(0.0..=1.0, value)
      color_well.rs     — color_well("#ff0000")
      popup.rs          — popup(["A", "B", "C"], selected)
      image.rs          — image("path") / sf_symbol("gear")
      stack.rs          — vstack![] / hstack![]
      spacer.rs         — spacer()
      separator.rs      — separator()
      scroll.rs         — scroll(content)
      split.rs          — split(sidebar, content)
      table.rs          — table(columns, rows)
      list.rs           — list(items, row_builder)
      group.rs          — group("Header", content)
      form.rs           — form(rows) — label-value pairs
      tab.rs            — tabs(["General", "Font"], content_for_tab)
      alert.rs          — alert("Title", "Message").show()
      sheet.rs          — sheet(content)
      popover.rs        — popover(content)
      context_menu.rs   — context_menu(items)
      toolbar.rs        — toolbar(items)
      search.rs         — searchable(content, query)
      reconciler.rs     — diff/patch engine for view updates
      state.rs          — @State-like reactive bindings
```

---

## Framework Coverage Checklist

### Priority 1 — Already needed by alleycat (port from ac-stray/ac-sill)

- [ ] **ObjC Runtime** (Layer 0) — msg_send!, sel!, cls!, class creation, ivars, protocols
- [ ] **CoreFoundation** — CFString, CFArray, CFDictionary, CFNumber, CFRef RAII, CFAttributedString, CFSet, CFURL
- [ ] **CoreGraphics** — CGContext, CGColorSpace, CGImage, CGRect/Point/Size, CGBitmapContextCreate, display info
- [ ] **CoreText** — CTFont (create, metrics, glyphs, draw, cascade), CTFontDescriptor, CTFontCollection, CTLine, CTRun
- [ ] **Metal** — MTLDevice, MTLCommandQueue, MTLCommandBuffer, MTLRenderCommandEncoder, MTLLibrary, MTLBuffer, MTLTexture, MTLRenderPipelineState, MTLSamplerState, MTLVertexDescriptor, MTLRenderPassDescriptor
- [ ] **QuartzCore** — CAMetalLayer, CAMetalDrawable
- [ ] **AppKit Core** — NSApplication, NSWindow, NSPanel, NSView, NSEvent, NSScreen, NSAppearance, NSCursor
- [ ] **AppKit Controls** — NSTextField, NSSearchField, NSButton, NSSlider, NSPopUpButton, NSColorWell, NSProgressIndicator, NSStepper
- [ ] **AppKit Views** — NSTableView, NSScrollView, NSSplitView, NSStackView, NSVisualEffectView, NSImageView, NSBox
- [ ] **AppKit Chrome** — NSMenu, NSMenuItem, NSToolbar, NSToolbarItem, NSFontPanel, NSFontManager
- [ ] **AppKit Services** — NSPasteboard, NSSharingServicePicker, NSUserActivity, NSWorkspace, NSUserDefaults
- [ ] **QuickLook** — QLPreviewPanel

### Priority 2 — Common needs, should have soon

- [ ] **AppKit Dialogs** — NSAlert, NSOpenPanel, NSSavePanel, NSColorPanel
- [ ] **AppKit Advanced** — NSOutlineView, NSCollectionView, NSTabView, NSDatePicker, NSPathControl, NSTokenField
- [ ] **AppKit Animation** — NSAnimationContext, NSViewAnimation
- [ ] **AppKit Layout** — NSLayoutConstraint, NSLayoutAnchor (Auto Layout builder API)
- [ ] **AppKit Drag & Drop** — NSDraggingInfo, drag protocols
- [ ] **CoreGraphics Extra** — CGPath, CGGradient, CGEventTap (global hotkeys), CGWindow (screenshots)
- [ ] **Metal Compute** — MTLComputePipelineState, MTLComputeCommandEncoder
- [ ] **Metal Advanced** — MTLHeap, MTLFence, MTLCaptureManager
- [ ] **QuartzCore Animation** — CAAnimation, CABasicAnimation, CATransaction, CADisplayLink
- [ ] **Foundation** — NSFileManager, NSTimer, NSTask, NSJSONSerialization
- [ ] **Dispatch (GCD)** — dispatch_queue, dispatch_async, dispatch_group, dispatch_semaphore
- [ ] **WebKit** — WKWebView (embedded web content)
- [ ] **Security** — Keychain access (SecItemAdd, SecItemCopyMatching)

### Priority 3 — Nice to have, build when needed

- [ ] **AVFoundation** — audio playback, speech synthesis, camera capture
- [ ] **IOKit** — HID device access, power management, USB
- [ ] **Network.framework** — NWConnection, NWListener (modern networking)
- [ ] **LocalAuthentication** — Touch ID / Face ID
- [ ] **MapKit** — MKMapView
- [ ] **StoreKit** — in-app purchases
- [ ] **Metal Ray Tracing** — MTLAccelerationStructure
- [ ] **AppKit Accessibility** — full VoiceOver / accessibility protocol support
- [ ] **AppKit Printing** — NSPrintOperation
- [ ] **AppKit Spell Check** — NSSpellChecker

### Declarative UI (Layer 3)

- [ ] View trait + reconciler (diff/patch)
- [ ] State management (@State-like reactive bindings)
- [ ] text, textfield, button, checkbox, slider, color_well, popup
- [ ] vstack, hstack, spacer, separator
- [ ] scroll, split, table, list
- [ ] group, form, tabs
- [ ] alert, sheet, popover, context_menu
- [ ] toolbar, searchable
- [ ] sf_symbol (SF Symbols integration)

---

## Binding Generation Strategy

Layers 0-1 are **pattern-based, not creative**. Every ObjC class binding follows the same shape:

1. Newtype around `Id`
2. Methods that call `msg_send!()` with the right selector and type signature
3. `Drop` impl that releases
4. Enums for constant sets
5. Builder if the class has many configuration options

This means bindings can be authored with automation — the patterns are known, the Apple headers define the selectors and types, and the output is deterministic Rust. The generated source is checked in, reviewed, and editable. No runtime codegen, no proc macros, no build-script surprises. Just plain `.rs` files that happen to have been stamped out rather than typed by hand.

The same applies to CoreFoundation types (RAII wrapper + typed accessors), Metal objects (newtype + descriptor builders), and CoreGraphics functions (thin wrappers with Rust types).

Layer 2 (ergonomic wrappers) and Layer 3 (declarative UI) require actual design work and are written by hand.

---

## Threading & Safety

- **AppKit** is main-thread-only. AppKit types are `!Send + !Sync`. The trampoline/callback system ensures closures run on the main thread.
- **Metal** has specific threading rules: `MTLDevice` is `Send + Sync`, command buffers are `Send` but not `Sync`, encoders are `!Send + !Sync`. Wrappers enforce this at the type level.
- **CoreFoundation / CoreGraphics** types are generally `Send` (they're refcounted C objects), but not `Sync` without external synchronization.
- **Dispatch (GCD)** is inherently concurrent — the Rust wrappers use closures with appropriate `Send` bounds on `dispatch_async`.
- Layer 0 (`msg_send!`, `Id`) makes no thread-safety guarantees. You're on your own, same as raw C.

## Error Handling

Apple APIs use a mix of error patterns. malus normalizes them:

- **`NSError**` out-params** become `-> Result<T, NSError>` in Rust.
- **Nil returns** (where nil means failure) become `-> Option<T>`.
- **Nil returns with NSError** become `-> Result<T, NSError>` (nil maps to Err).
- **ObjC exceptions** are not caught. Apple's own documentation says exceptions are for programmer error, not recoverable conditions. If you hit one, it's a bug.
- Infallible Apple APIs that always succeed stay as plain return types, no wrapping.

---

## Design Principles

### 1. Every binding is a newtype

```rust
// Layer 1 — typed wrapper
pub struct NSWindow(Id);

impl NSWindow {
    pub fn new(rect: CGRect, style: u64) -> Self { ... }
    pub fn set_title(&self, title: &str) { ... }
    pub fn make_key_and_order_front(&self) { ... }
    pub fn content_view(&self) -> NSView { ... }
    pub fn close(&self) { ... }
}

impl Drop for NSWindow {
    fn drop(&mut self) { release(self.0); }
}
```

### 2. Closures, not delegates

```rust
// Wrap ObjC delegate patterns into Rust closures
window.on_close(|| println!("bye"));
button.on_click(|| save_config());
text_field.on_change(|new_text| update_search(new_text));
slider.on_change(|value| set_opacity(value));
```

### 3. Enums for Apple's magic numbers

```rust
// Not this
f_set_int(view, sel!("setMaterial:"), 13);

// This
view.set_material(Material::HUDWindow);
```

### 4. Builder patterns

```rust
let window = NSWindow::builder()
    .title("Settings")
    .size(700, 500)
    .min_size(600, 400)
    .style(WindowStyle::TITLED | WindowStyle::CLOSABLE | WindowStyle::RESIZABLE)
    .appearance(Appearance::DarkAqua)
    .toolbar(Toolbar::new("main").style(ToolbarStyle::UnifiedCompact))
    .build();
```

### 5. Raw escape hatch always available

```rust
// If we haven't wrapped something yet, drop to Layer 0
let obj = window.raw(); // get the Id
msg_send!(obj, sel!("setSomeObscureThing:"), fn(Id, Sel, Id) -> (), value);
```

### 6. Feature flags for framework opt-in

```toml
[dependencies]
malus = { version = "0.1", features = ["appkit", "metal"] }

# Or everything
malus = { version = "0.1", features = ["full"] }
```

Feature flags are per-framework (`appkit`, `metal`, `core-text`, `webkit`, etc.), not per-class. Finer granularity isn't worth the maintenance cost — Cargo features compound combinatorially.

---

## What Exists Today (v0.1)

- Layer 0: ObjC runtime (msg_send!, sel!, cls!, class creation, ivars) — **done**
- Layer 0: Geometry types (CGRect, CGPoint, CGSize) — **done**
- Layer 0: NSString bridge — **done**
- Layer 0: BOOL helpers (aarch64 vs x86_64) — **done**
- Events: Trampoline system for target-action and delegate callbacks — **done**
- Layer 3: View trait — **done** (build only, no reconciler)
- Layer 3: text, textfield, button, vstack, hstack, spacer — **done**
- App bootstrap: NSApplication + NSWindow + event loop — **done**

Everything else is TODO.

---

## Migration Path

Once malus has the Layer 1 bindings for AppKit/Metal/CoreGraphics/CoreText:

1. **ac-stray** (alleycat's ObjC bindings) gets replaced by `malus::runtime` + `malus::core_foundation` + `malus::core_graphics` + `malus::core_text`
2. **ac-sill** (alleycat's windowing) gets replaced by `malus::appkit`
3. **ac-metal** (alleycat's Metal renderer) uses `malus::metal`
4. **ac-board** (alleycat's clipboard) gets replaced by `malus::appkit::pasteboard`
5. **ac-font** (alleycat's font system) uses `malus::core_text` + `malus::core_graphics`
6. **settings.rs** (1100 lines of raw ObjC) becomes ~100 lines of Layer 3 declarative UI

The existing raw ObjC code in alleycat is the proof-of-concept for what malus wraps. Every pattern already exists — malus just makes it reusable.
