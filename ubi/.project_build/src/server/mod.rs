
use std::collections::HashMap;
use lazy_static::lazy_static;
use may::net::TcpStream;
use std::io;

use crate::PgConnection;

#[cfg(feature = "ws")]
use may_minihttp::WsContext;

pub mod api;

pub type HandlerFn = fn(Option<&PgConnection>, may_minihttp::Request) -> Result<String, may_postgres::Error>;
pub type HandlerFn2 = fn(Option<&PgConnection>, may_minihttp::Request, &HashMap<String, String>) -> Result<String, may_postgres::Error>;

#[cfg(feature = "ws")]
pub type WsOnConnectHandler = fn(&str, &mut TcpStream, &mut WsContext) -> io::Result<()>;

#[cfg(feature = "ws")]
pub type WsOnMessageHandler = fn(&str, &mut TcpStream, &mut WsContext, Option<&str>) -> io::Result<()>;

#[cfg(feature = "ws")]
pub type WsOnCloseHandler = fn(&str, &mut TcpStream, &mut WsContext, u16, &str) -> io::Result<()>;

#[cfg(not(feature = "ws"))]
lazy_static! {
    pub static ref ROUTES: HashMap<&'static str, HandlerFn> = {
        let mut map = HashMap::new();
        map.insert("/api/get", api::get as HandlerFn);
        map.insert("/api/post", api::post as HandlerFn);
        map.insert("/api/delete", api::delete as HandlerFn);;
        map
    };

    pub static ref PARAMETERIZED_ROUTES: HashMap<&'static str, HandlerFn2> = {
        let mut map = HashMap::new();
        ;
        map
    };

    pub static ref FRONTEND_PARAMETERIZED_ROUTES: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        ;
        map
    };

}

#[cfg(feature = "ws")]
lazy_static! {
    pub static ref ROUTES: HashMap<&'static str, HandlerFn> = {
        let mut map = HashMap::new();
        map.insert("/api/get", api::get as HandlerFn);
        map.insert("/api/post", api::post as HandlerFn);
        map.insert("/api/delete", api::delete as HandlerFn);;
        map
    };

    pub static ref PARAMETERIZED_ROUTES: HashMap<&'static str, HandlerFn2> = {
        let mut map = HashMap::new();
        ;
        map
    };

    pub static ref WS_ONCONNECT_ROUTES: HashMap<&'static str, WsOnConnectHandler> = {
        let mut map = HashMap::new();
        
        map
    };

    pub static ref WS_ONMESSAGE_ROUTES: HashMap<&'static str, WsOnMessageHandler> = {
        let mut map = HashMap::new();
        
        map
    };

    pub static ref WS_ONCLOSE_ROUTES: HashMap<&'static str, WsOnCloseHandler> = {
        let mut map = HashMap::new();
        
        map
    };
}
