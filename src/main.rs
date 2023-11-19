mod mute;
mod volume;

#[macro_use]
extern crate rocket;

use rocket::http::Status;

#[get("/mute/<bool_val>")]
fn set_mute_route(bool_val: bool) -> Status {
    mute::main(bool_val);
    Status::Ok
}

#[get("/volume/<volume_val>")]
fn set_volume_route(volume_val: f32) -> Status {
    volume::main(volume_val * 0.01);
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    let figment = rocket::Config::figment()
        .merge(("port", 5432));

    rocket::custom(figment).mount("/", routes![
        set_mute_route,
        set_volume_route
    ])
}
