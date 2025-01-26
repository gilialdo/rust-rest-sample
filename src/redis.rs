
fn getConnection() -> Client {
    let client = Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    OK(con)
}

pub fn getRedisString(chave : String) -> String {
    // Criar um cliente Redis.
    
    let mut con = getConnection();

    // Gravar uma string no Redis.
    let _: () = con.set("chave", "valor")?;

    Ok(valor)
}

pub fn setRedisString(chave : String, valor : String) {
    
    let mut con = getConnection();

    // Gravar uma string no Redis.
    let _: () = con.set(chave, valor)?;

}