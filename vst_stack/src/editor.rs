use cocoa::appkit::{NSTextField, NSView};
use cocoa::base::nil;
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::runtime::Object;
use vst;

pub struct Editor {
    pub path: String,
    pub is_open: bool,
}

impl vst::editor::Editor for Editor {
    fn size(&self) -> (i32, i32) {
        (500, 24)
    }

    fn position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn open(&mut self, parent: *mut std::ffi::c_void) -> bool {
        if self.is_open {
            return self.is_open;
        };
        let view = parent as *mut Object;
        let (x, y) = self.position();
        let (w, h) = self.size();
        unsafe {
            let text_field = NSTextField::alloc(nil);
            let _: () = msg_send![
                text_field,
                initWithFrame:
                    NSRect::new(
                        NSPoint::new(f64::from(x), f64::from(y)),
                        NSSize::new(f64::from(w), f64::from(h)),
                    )
            ];
            let _: () = msg_send![
                text_field,
                setStringValue: NSString::alloc(nil).init_str(&self.path)
            ];
            let _: () = msg_send![text_field, autorelease];
            view.addSubview_(text_field);
        }
        self.is_open = true;
        return self.is_open;
    }

    fn close(&mut self) {
        self.is_open = false;
    }

    fn is_open(&mut self) -> bool {
        self.is_open
    }
}
