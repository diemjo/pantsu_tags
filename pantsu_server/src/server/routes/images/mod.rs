use rocket::Route;

mod image;
mod images;
mod import;
mod tags;

pub fn get_routes() -> Vec<Route> {
    return vec![
        image::get_routes(),
        images::get_routes(),
        import::get_routes(),
        tags::get_routes(),
    ].concat()
}
