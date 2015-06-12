extern crate nickel;
extern crate mysql;
extern crate plugin;
extern crate typemap;

pub use middleware::{ MysqlMiddleware, MysqlRequestExtensions };

mod middleware;
