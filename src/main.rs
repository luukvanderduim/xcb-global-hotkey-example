use xcb;
use xcb::ffi::xproto::*;
use xcb::xkb;
//use xcb::xproto;
use xcb_util::keysyms::KeySymbols;


fn main() {

    let (conn, _) = xcb::Connection::connect(None).unwrap();
   
    let key_symb: KeySymbols = KeySymbols::new(&conn);
    let mut keyf12 = key_symb.get_keycode(65481 as u32);

    let setup = conn.get_setup();
    
    for screen in setup.roots() {
            let mut rez_grab = xcb::grab_key(&conn, true, screen.root(), keyf12.next().unwrap() as u16 , 0, XCB_GRAB_MODE_ASYNC as u8, XCB_GRAB_MODE_ASYNC as u8);
            let valuelist = [(XCB_EVENT_MASK_EXPOSURE , XCB_EVENT_MASK_BUTTON_PRESS)];
            let mut _rez_chatrr = xcb::xproto::change_window_attributes(&conn, screen.root(), &valuelist );
    }

    conn.flush();
    loop {
        if let Some(ev) = (&conn).wait_for_event() {
            if ev.response_type() & !0x80 == XCB_KEY_PRESS {
                println!("Crtl-F12, Really?!");
            }
        }
    }
}