mod sql;
use std::process::exit;

use sql::Db;

fn main() {
    let mut _db: Db = Db {
        database_type: "mysql".to_owned(),
        database_name: "file".to_owned(),
        database_ip: "127.0.0.1".to_owned(),
        database_port: 4466,
        password: "SQS".to_owned(),
        username: "root".to_owned(),
    };
    Db::new(&_db);
    Db::creat_table(&mut _db, 1);
    exit(0);
}
