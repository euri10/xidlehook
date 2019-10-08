//! Clone of `xprintidle` in rust

use xidlehook::{modules::Xcb, Result};

fn main() -> Result<()> {
    let xcb = Xcb::new()?;
    let idle = xcb.get_idle()?;

    println!("{}", idle.as_millis());

    Ok(())
}
