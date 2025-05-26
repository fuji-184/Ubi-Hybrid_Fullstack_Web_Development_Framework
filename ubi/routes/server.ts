// name api, about tessss, 200 berhasill, 500 error, body produk
function get(): string {
    let produk: produk = ubi.query("select * from produk;", null);

    return ubi.json(produk);
}

// name api, about tessss, 200 berhasill, 500 error, body produk
function post(): string {
    let data: produk = ubi.req.data;
    ubi.query("insert into produk(nama, harga) values($1, $2);", [data.nama, data.harga]);
    return ubi.json(data);
}

function delete(): string {
    let id: id = ubi.req.data;
    ubi.query("delete from produk where id = $1;", [id.id]);
    return ubi.json("berhasil");
}
