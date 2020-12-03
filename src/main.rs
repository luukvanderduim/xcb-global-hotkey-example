use std::error::Error;
use x11rb::{
    connect,
    connection::Connection,
    protocol::{
        xproto::{grab_key, ConnectionExt, ChangeWindowAttributesAux, EventMask, GrabMode, ModMask, Setup},
        Event,
    },
};

fn main() -> Result<(), Box<dyn Error>> {
    let (conn, screen_num) = connect(None)?;
    let setup: &Setup = conn.setup();
    let rootwindow = setup.roots[screen_num].root;

    let kpress = ChangeWindowAttributesAux::default()
    .event_mask(EventMask::KeyPress);
    
    // A Display refers to a server, A display may have more than one screen but it is not that common
    // (Screens are not the Workspaces (desktops) as I once thought. https://unix.stackexchange.com/questions/503806/what-are-x-server-display-and-screen )
    
    let valid_strokemods: [u16; 4] = [2 | 16 | 4, 2 | 4, 16 | 4, 4];
    for strokemod in valid_strokemods.iter() {
        grab_key(&conn, false, rootwindow, *strokemod, 25, GrabMode::Async, GrabMode::Async )?
        .check()?;
    }
    

    &mut conn.change_window_attributes(rootwindow, &kpress)?;
    
    &conn.flush()?;

    let windowevent = &conn.wait_for_event()?;
        
    if let Event::KeyPress(kpe) = windowevent {
        println!("Keycode: {}", &kpe.detail);
    } else {
        println!("Were we subscribed to such event?! {:?}", &windowevent);
    }

    Ok(())
}
