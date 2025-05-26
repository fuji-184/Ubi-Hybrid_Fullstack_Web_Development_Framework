use std::io::BufRead;
#[utoipa::path(
    get,
    path = "/api",
    responses(
        (status = 200, description = "berhasill", body = produk),
        (status = 500, description = "error")
    ),
    tag = "api"
)]
pub fn get(db: Option<&crate::PgConnection>, req: may_minihttp::Request) -> Result<String, may_postgres::Error> {
  let db = db.unwrap();
                
    let produk = db.query("select * from produk;", None)?; return Ok(serde_json::json!(&produk).to_string());}


#[utoipa::path(
    post,
    path = "/api",
    responses(
        (status = 200, description = "berhasill", body = produk),
        (status = 500, description = "error")
    ),
    tag = "api"
)]
pub fn post(db: Option<&crate::PgConnection>, req: may_minihttp::Request) -> Result<String, may_postgres::Error> {
  let db = db.unwrap();
                
    let data: produk = serde_json::from_slice(req.body().fill_buf().unwrap()).unwrap();
    db.query("insert into produk(nama, harga) values($1, $2);", Some(&[&data.nama, &data.harga]))?; return Ok(serde_json::json!(&data).to_string());}

pub fn delete(db: Option<&crate::PgConnection>, req: may_minihttp::Request) -> Result<String, may_postgres::Error> {
  let db = db.unwrap();
                
    let id: id = serde_json::from_slice(req.body().fill_buf().unwrap()).unwrap();
    db.query("delete from produk where id = $1;", Some(&[&id.id]))?; return Ok(serde_json::json!(&"berhasil".to_string()).to_string());}#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
        pub struct produk { id: i32, // primary_key
nama: String,
harga: i32, }#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
        pub struct id { id: i32, }
