use std::default::Default;
use std::sync::Arc;

use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;
use mysql::value::from_value;
use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use typemap::Key;
use plugin::{Pluggable, Extensible};

pub struct MysqlMiddleware {
    pub pool: Arc<MyPool>,
}

impl MysqlMiddleware {
    pub fn new(db_name: &str, user: &str, pass: &str) -> MysqlMiddleware {
        let options = MyOpts {
                user: Some(user.into()),
                pass: Some(pass.into()),
                db_name: Some(db_name.into()),
                ..Default::default()
        };
        let pool = MyPool::new(options).unwrap();
        MysqlMiddleware {
            pool: Arc::new(pool),
        }
    }
}

impl Key for MysqlMiddleware { type Value = Arc<MyPool>; }

impl Middleware for MysqlMiddleware {
    fn invoke<'res>(&self, request: &mut Request, response: Response<'res>) -> MiddlewareResult<'res> {
        request.extensions_mut().insert::<MysqlMiddleware>(self.pool.clone());
        Ok(Continue(response))
    }    
}

pub trait MysqlRequestExtensions {
    fn db_connection(&self) -> Arc<MyPool>;    
}

impl<'a, 'b, 'c> MysqlRequestExtensions for Request<'a, 'b, 'c> {
    fn db_connection(&self) -> Arc<MyPool> {
        self.extensions().get::<MysqlMiddleware>().unwrap().clone()
    }    
}
