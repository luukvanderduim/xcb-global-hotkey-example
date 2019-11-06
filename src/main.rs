use xcb;
use xcb::ffi::xproto::*;
use xcb::xproto::*;
use xcb::xkb;
use xcb_util::keysyms::KeySymbols;
use x11::keysym::*;

fn main() {

    let (conn, screen_num) = xcb::Connection::connect(None).expect("Connection failed");
    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).expect("screen setup failed");
   
  //  let key_symb: KeySymbols = KeySymbols::new(&conn);
  //  let mut key = key_symb.get_keycode(xcb::Keysym::default());
    
  //  for screen in setup.roots() {

            let _ = xcb::grab_key_checked(&conn, true, screen.root(), MOD_MASK_ANY as u16, 96 as u8, GRAB_MODE_ASYNC as u8, GRAB_MODE_ASYNC as u8)
            .request_check().expect("The key grab failed");
            let valuelist = [( CW_EVENT_MASK , EVENT_MASK_KEY_PRESS)];
            let _rez_chatrr = xcb::xproto::change_window_attributes_checked(&conn, screen.root(), &valuelist )
            .request_check().expect("Change of window attributes failed");
                   
  //  }

   
    loop {
         &conn.flush();
        if let Some(ev) = (&conn).wait_for_event() {
            if ev.response_type() & !0x80 == KEY_PRESS {
                println!("Key event");
            } 
                        
        }
    }
}