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
        let pool = Pool::new(Opts::from_url(&connect_sql).unwrap()).unwrap();
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

    pub fn creat_table(&mut self, idx: i64) {
        let creat_table_sql: &str= &format!("SET FOREIGN_KEY_CHECKS=0;
            SET SQL_MODE = \"NO_AUTO_VALUE_ON_ZERO\";
            START TRANSACTION;
            SET time_zone = \"+00:00\";
            CREATE DATABASE IF NOT EXISTS `file` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_520_ci;
            USE `file`;
            CREATE TABLE `file_data_{}` (
            `id` bigint(20) UNSIGNED NOT NULL COMMENT '文件ID',
            `data` longblob NOT NULL DEFAULT '' COMMENT '文件数据'
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_520_ci;
            CREATE TABLE `file_path_{}` (
            `path` varchar(535) NOT NULL COMMENT '文件路径',
            `isdel` bigint(20) NOT NULL DEFAULT 0 COMMENT '是否删除',
            `userid` bigint(20) UNSIGNED NOT NULL DEFAULT 0 COMMENT '用户ID',
            `file_id` bigint(20) UNSIGNED NOT NULL DEFAULT 0 COMMENT '文件ID',
            `time` datetime NOT NULL DEFAULT current_timestamp() COMMENT '上传时间'
            ) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_520_ci;
            ALTER TABLE `file_data`
            ADD PRIMARY KEY (`id`);
            ALTER TABLE `file_path`
            ADD PRIMARY KEY (`path`),
            ADD KEY `userid` (`userid`),
            ADD KEY `file_id` (`file_id`),
            ADD KEY `time` (`time`),
            ADD KEY `isdel` (`isdel`);
            ALTER TABLE `file_data`
            MODIFY `id` bigint(20) UNSIGNED NOT NULL AUTO_INCREMENT COMMENT '文件ID';
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
