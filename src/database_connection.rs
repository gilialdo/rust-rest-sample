

pub async fn connect() -> core::result::Result<sqlx::Pool<sqlx::MySql>, sqlx::Error> {
    return sqlx::MySqlPool::connect("mysql://user:pass@localhost:3306/database").await;
}

pub struct DadosConexao {
    pub endereco : String,
    pub port : String,
    pub usuario : String,
    pub pass : String,
    pub database : String,
}

pub async fn get_connection(dadosConexao: DadosConexao ) -> core::result::Result<sqlx::Pool<sqlx::MySql>, sqlx::Error> {
    return get_connection_param_str(dadosConexao.endereco, dadosConexao.port, dadosConexao.usuario, dadosConexao.pass, dadosConexao.database).await;
}

async fn get_connection_param_str(endereco : String, port : String, usuario : String, pass : String, database : String) -> core::result::Result<sqlx::Pool<sqlx::MySql>, sqlx::Error> {
    let endereco : String = format!("mysql://{}:{}@{}:{}/{}", usuario, pass, endereco, port, database);
    //  "mysql://" + usuario + ":" + pass + "@" + endereco + ":" + port + "/" + database;
    return sqlx::MySqlPool::connect(&endereco).await;
}