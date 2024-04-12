SET FOREIGN_KEY_CHECKS=0;
SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
START TRANSACTION;
SET time_zone = "+00:00";
CREATE DATABASE IF NOT EXISTS `file` DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_520_ci;
USE `file`;
CREATE TABLE `file_data` (
  `id` bigint(20) UNSIGNED NOT NULL COMMENT '文件ID',
  `data` longblob NOT NULL DEFAULT '' COMMENT '文件数据'
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_520_ci;
CREATE TABLE `file_path` (
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
