use deadpool_postgres::Config;


pub fn pg_conf() -> Config {
  let mut cfg = Config::new();
  cfg.host = Some("127.0.0.1".to_string());
  cfg.port = Some(5432);
  cfg.user = Some("sm_owner".to_string());
  cfg.password = Some("password".to_string());
  cfg.dbname = Some("sm_db".to_string());
  cfg
}