use rocket::Route;

mod images;
mod sauce;
mod tags;

pub fn get_routes() -> Vec<Route> {
    return vec![
        images::get_routes(),
        sauce::get_routes(),
        tags::get_routes(),
    ].concat()
}
