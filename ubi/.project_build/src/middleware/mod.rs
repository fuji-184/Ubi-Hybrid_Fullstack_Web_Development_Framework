

use lazy_static::lazy_static;
use std::collections::HashMap;

        
pub type MiddlewareFn = fn(&mut may_minihttp::Response) -> bool;

lazy_static! {
    pub static ref MIDDLEWARE_ROUTES: HashMap<&'static str, MiddlewareFn> = {
        let mut map = HashMap::new();
        ;
        map
    };

}



    