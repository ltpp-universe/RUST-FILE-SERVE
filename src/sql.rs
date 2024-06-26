use mysql::{prelude::Queryable, *};
static mut CONNECTION: Option<PooledConn> = None;

pub struct Db {
    pub(crate) database_type: String,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) database_ip: String,
    pub(crate) database_port: i64,
    pub(crate) database_name: String,
}

impl Db {
    pub fn connect(&mut self) -> Self {
        let connect_sql = format!(
            "{}://{}:{}@{}:{}/{}",
            self.database_type,
            self.username,
            self.password,
            self.database_ip,
            self.database_port,
            self.database_name
        );
        let pool: Pool = Pool::new(Opts::from_url(&connect_sql).unwrap()).unwrap();
        unsafe {
            CONNECTION = Some(pool.get_conn().unwrap());
        }
        Self {
            database_type: self.database_type.clone(),
            username: self.username.clone(),
            password: self.password.clone(),
            database_ip: self.database_ip.clone(),
            database_port: self.database_port.clone(),
            database_name: self.database_name.clone(),
        }
    }

    pub fn close(&mut self) {
        unsafe {
            if let Some(conn) = &mut CONNECTION {
                drop(conn);
            }
        }
    }

    pub fn creat_db(&mut self) {
        let creat_db_sql: &str = "CREATE DATABASE IF NOT EXISTS `file`;";
        unsafe {
            if let Some(conn) = &mut CONNECTION {
                conn.query_drop(creat_db_sql).unwrap();
            }
        }
    }

    pub fn creat_table(&mut self, idx: i64) {
        let creat_table_sql: &str= &format!("SET FOREIGN_KEY_CHECKS=0;
            SET SQL_MODE = \"NO_AUTO_VALUE_ON_ZERO\";
            START TRANSACTION;
            SET time_zone = \"+00:00\";
            CREATE DATABASE IF NOT EXISTS `file` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_520_ci;
            USE `file`;
           CREATE TABLE `{}_file_data` (
                `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT COMMENT \'文件ID\',
                `data` longblob NOT NULL DEFAULT \'\' COMMENT \'文件数据\',
                PRIMARY KEY (`id`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
            CREATE TABLE `{}_file_path` (
                `path` varchar(535) NOT NULL COMMENT \'文件路径\',
                `isdel` bigint(20) NOT NULL DEFAULT 0 COMMENT \'是否删除\',
                `userid` bigint(20) UNSIGNED NOT NULL DEFAULT 0 COMMENT \'用户ID\',
                `file_id` bigint(20) UNSIGNED NOT NULL DEFAULT 0 COMMENT \'文件ID\',
                `time` datetime NOT NULL DEFAULT current_timestamp() COMMENT \'上传时间\',
                PRIMARY KEY (`path`),
                INDEX `' . $index . '_isdel` (`isdel`),
                INDEX `' . $index . '_userid` (`userid`),
                INDEX `' . $index . '_file_id` (`file_id`),
                INDEX `' . $index . '_time` (`time`)
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
            SET FOREIGN_KEY_CHECKS=1;
            COMMIT;
            ", idx , idx);
        unsafe {
            if let Some(conn) = &mut CONNECTION {
                conn.query_drop(creat_table_sql).unwrap();
            }
        }
    }
}
