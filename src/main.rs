// Copyright (c) 2024 fawn
// SPDX-License-Identifier: Apache-2.0

use xcb::{x, Connection};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let mut child: std::process::Child;

    if args.len() > 1 {
        child = std::process::Command::new(&args[1])
            .args(&args[2..])
            .spawn()?;
    } else {
        std::process::exit(0);
    }

    let (conn, _screen) = Connection::connect(None)?;
    let win = conn
        .wait_for_reply(conn.send_request(&x::GetInputFocus {}))?
        .focus();

    let unmap_cookie = conn.send_request_checked(&x::UnmapWindow { window: win });
    conn.check_request(unmap_cookie)?;

    let exit_code = child.wait()?.code().unwrap_or(1);

    let map_cookie = conn.send_request_checked(&x::MapWindow { window: win });
    conn.check_request(map_cookie)?;

    std::process::exit(exit_code)
}
