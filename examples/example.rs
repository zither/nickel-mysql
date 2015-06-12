#[macro_use] extern crate nickel;

extern crate mysql;
extern crate nickel_mysql;

use nickel::{Nickel, HttpRouter};
use mysql::value::from_value;
use nickel_mysql::{MysqlMiddleware, MysqlRequestExtensions};

struct User {
    id: u32,
    email: String,
}

fn main() {
    let mut app = Nickel::new();

    app.utilize(MysqlMiddleware::new("db_name", "user", "pass"));

    app.get("/conn", middleware! { |request|
        let connection = request.db_connection();
        // use connection
        let users: Vec<User> = connection.prepare("SELECT id, email FROM user")
        .and_then(|mut stmt| {
            stmt.execute(&[]).map(|result| {
                result.map(|x| x.unwrap()).map(|row| {
                    User {
                        id: from_value(&row[0]),
                        email: from_value(&row[1]),
                    }
                }).collect()    
            })    
        }).unwrap();

        for user in users.iter() {
            println!("id: {}, email: {}", user.id, user.email);    
        }
    });

    app.listen("127.0.0.1:9001");
}
