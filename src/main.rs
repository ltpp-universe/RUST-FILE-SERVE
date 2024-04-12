mod sql;

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
    _db.connect();
    _db.creat_table(1);
    _db.close();
    return;
}
