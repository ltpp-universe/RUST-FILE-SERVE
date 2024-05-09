mod sql;

use sql::Db;

fn main() {
    let mut _db: Db = Db {
        database_type: "mysql".to_owned(),
        database_name: "file".to_owned(),
        database_ip: "ltpp.vip".to_owned(),
        database_port: 60002,
        password: "ltpp".to_owned(),
        username: "root".to_owned(),
    };
    _db.connect();
    for idx in 1..=10 {
        _db.creat_table(idx);
    }
    _db.close();
    return;
}
