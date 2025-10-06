pub mod authorization;
pub mod user;

use rocket::Route;

pub fn all_routes() -> Vec<Route> {
    [user::routes(), authorization::routes()].concat()
}
