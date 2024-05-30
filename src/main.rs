#[macro_use] extern crate rocket;

use rocket::http::Status;
use std::process::Command;
use std::path::PathBuf;

static NIRCMDEXE: &[u8] = include_bytes!("../bin/nircmdc.exe");

const MAX_VOLUME: i32 = 65535;

fn get_nircmd_path() -> PathBuf {
    let nircmd_path = std::env::temp_dir().join("nircmdc.exe");
    std::fs::write(&nircmd_path, NIRCMDEXE).expect("Failed to write nircmd.exe");
    nircmd_path
}

#[get("/mute/<value>")]
fn set_mute_route(value: bool) -> Status {
    let mute_value = if value { "1" } else { "0" };

    Command::new(get_nircmd_path())
        .arg("mutesysvolume")
        .arg(mute_value)
        .spawn()
        .expect("Failed to mute volume")
    ;
    Status::Ok
}

#[get("/volume/<value>")]
fn set_volume_route(value: i32) -> Status {
    if value < 0 || value > 100 {
        return Status::BadRequest;
    }
    let volume_value = (value * MAX_VOLUME / 100) as i32;

    Command::new(get_nircmd_path())
        .arg("setsysvolume")
        .arg(volume_value.to_string())
        .spawn()
        .expect("Failed to set volume")
    ;
    Status::Ok
}

#[get("/shutdown")]
fn set_shutdown_route() -> Status {
    Command::new(get_nircmd_path())
        .arg("exitwin")
        .arg("poweroff")
        .spawn()
        .expect("Failed to set volume")
    ;
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 5432))
        .merge(("address", "0.0.0.0"))
    ;

    rocket::custom(figment).mount("/", routes![
        set_mute_route,
        set_volume_route,
        set_shutdown_route
    ])
}
