-- MySQL dump 10.13  Distrib 8.0.19, for Win64 (x86_64)
--
-- Host: 192.168.16.136    Database: rust-admin
-- ------------------------------------------------------
-- Server version	8.0.41-0ubuntu0.22.04.1

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `system_admin_log`
--

DROP TABLE IF EXISTS `system_admin_log`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_admin_log` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `system_user_id` int NOT NULL DEFAULT '0' COMMENT '用户ID',
  `system_real_name` varchar(32) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '操作人的真实姓名',
  `path` varchar(64) NOT NULL DEFAULT '' COMMENT '路径',
  `method` varchar(16) NOT NULL DEFAULT '' COMMENT '请求方法',
  `request_url` varchar(256) NOT NULL COMMENT '请求数据',
  `request_body` json DEFAULT NULL COMMENT '请求主体',
  `response_data` json DEFAULT NULL COMMENT '返回数据',
  `ip` varchar(32) NOT NULL DEFAULT '' COMMENT 'ip信息',
  `user_agent` varchar(512) NOT NULL DEFAULT '' COMMENT '浏览器信息',
  `response_time_ms` bigint NOT NULL DEFAULT '0' COMMENT '从请求到返回的毫秒数',
  `response_status` int unsigned NOT NULL DEFAULT '0' COMMENT '返回的http状态',
  `operate_name` varchar(64) NOT NULL DEFAULT '' COMMENT '操作名称',
  `error_message` varchar(256) NOT NULL DEFAULT '' COMMENT '错误信息',
  `created_at` int unsigned NOT NULL DEFAULT '0' COMMENT '创建时间',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=166 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='操作日志';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_admin_log`
--

LOCK TABLES `system_admin_log` WRITE;
/*!40000 ALTER TABLE `system_admin_log` DISABLE KEYS */;
INSERT INTO `system_admin_log` VALUES (1,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',31,200,'操作日志列表','',1742133252),(2,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',13,200,'操作日志列表','',1742133254),(3,1,'吴俊','/adm/system/config/list','GET','/system/config/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',33,200,'获取系统配置项','',1742133259),(4,1,'吴俊','/adm/system/dictionary/list','GET','/system/dictionary/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',47,200,'获取字典列表','',1742133263),(5,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取角色列表','',1742133273),(6,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取菜单列表','',1742133370),(7,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',40,200,'获取系统用户列表','',1742133374),(8,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取角色列表','',1742133375),(9,1,'吴俊','/adm/system/dictionary/list','GET','/system/dictionary/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取字典列表','',1742133376),(10,1,'吴俊','/adm/system/config/list','GET','/system/config/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取系统配置项','',1742133377),(11,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',12,200,'操作日志列表','',1742133379),(12,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取登录用户信息','',1742133564),(13,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',72,200,'获取用户分配的菜单列表','',1742133564),(14,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'操作日志列表','',1742133566),(15,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取登录用户信息','',1742133574),(16,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取用户分配的菜单列表','',1742133574),(17,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',14,200,'操作日志列表','',1742133575),(18,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取登录用户信息','',1742133675),(19,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',5,200,'获取用户分配的菜单列表','',1742133675),(20,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',13,200,'操作日志列表','',1742133677),(21,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',14,200,'获取登录用户信息','',1742133796),(22,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',169,200,'获取用户分配的菜单列表','',1742133796),(23,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',14,200,'操作日志列表','',1742133797),(24,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',25,200,'操作日志列表','',1742133829),(25,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',15,200,'获取登录用户信息','',1742133890),(26,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取用户分配的菜单列表','',1742133890),(27,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'操作日志列表','',1742133892),(28,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取登录用户信息','',1742133904),(29,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取用户分配的菜单列表','',1742133905),(30,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'操作日志列表','',1742133907),(31,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取登录用户信息','',1742134052),(32,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取用户分配的菜单列表','',1742134052),(33,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'操作日志列表','',1742134052),(34,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取登录用户信息','',1742134173),(35,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',144,200,'获取用户分配的菜单列表','',1742134173),(36,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',17,200,'操作日志列表','',1742134201),(37,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'操作日志列表','',1742134212),(38,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取登录用户信息','',1742134222),(39,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取用户分配的菜单列表','',1742134222),(40,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',27,200,'操作日志列表','',1742134223),(41,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取登录用户信息','',1742134243),(42,1,'吴俊','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'未知操作','',1742134243),(43,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取用户分配的菜单列表','',1742134243),(44,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',17,200,'获取登录用户信息','',1742134254),(45,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',3,200,'获取用户分配的菜单列表','',1742134254),(46,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取菜单列表','',1742134255),(47,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取登录用户信息','',1742134321),(48,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取用户分配的菜单列表','',1742134321),(49,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取菜单列表','',1742134322),(50,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',27,200,'获取登录用户信息','',1742134333),(51,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',149,200,'获取用户分配的菜单列表','',1742134333),(52,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取菜单列表','',1742134333),(53,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取接口列表','',1742134336),(54,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取系统用户列表','',1742134337),(55,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',13,200,'获取角色列表','',1742134338),(56,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取登录用户信息','',1742134385),(57,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',97,200,'获取用户分配的菜单列表','',1742134385),(58,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',13,200,'获取角色列表','',1742134386),(59,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',23,200,'获取登录用户信息','',1742134407),(60,1,'吴俊','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',80,200,'未知操作','',1742134407),(61,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取用户分配的菜单列表','',1742134407),(62,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取登录用户信息','',1742134414),(63,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取用户分配的菜单列表','',1742134414),(64,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取菜单列表','',1742134415),(65,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取登录用户信息','',1742134565),(66,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',105,200,'获取用户分配的菜单列表','',1742134565),(67,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取菜单列表','',1742134567),(68,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取登录用户信息','',1742134573),(69,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',194,200,'获取用户分配的菜单列表','',1742134574),(70,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取菜单列表','',1742134574),(71,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',27,200,'获取登录用户信息','',1742134589),(72,1,'吴俊','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',92,200,'未知操作','',1742134589),(73,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取用户分配的菜单列表','',1742134589),(74,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取用户分配的菜单列表','',1742134593),(75,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',14,200,'获取菜单列表','',1742134594),(76,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取接口列表','',1742134613),(77,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取系统用户列表','',1742134614),(78,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取用户分配的菜单列表','',1742135516),(79,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取系统用户列表','',1742135516),(80,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取用户分配的菜单列表','',1742135580),(81,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',14,200,'获取系统用户列表','',1742135582),(82,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取接口列表','',1742135594),(83,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',5,200,'获取菜单列表','',1742135595),(84,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取接口列表','',1742136012),(85,1,'吴俊','/adm/system/api/save','POST','/system/api/save','{\"id\": 1, \"path\": \"/adm/system/menu/save\", \"group\": \"系统管理\", \"method\": \"POST\", \"api_key\": \"system:menu:save\", \"description\": \"保存菜单\"}','{\"code\": 200, \"data\": {\"id\": 1, \"path\": \"/adm/system/menu/save\", \"group\": \"系统管理\", \"method\": \"POST\", \"api_key\": \"system:menu:save\", \"created_at\": 1739199895, \"updated_at\": 1742136018, \"description\": \"保存菜单\"}, \"message\": \"success\"}','','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',50,200,'保存接口','',1742136018),(86,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',12,200,'获取接口列表','',1742136019),(87,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取用户分配的菜单列表','',1742140059),(88,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',18,200,'获取接口列表','',1742140059),(89,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取菜单列表','',1742140061),(90,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取接口列表','',1742140063),(91,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',40,200,'获取系统用户列表','',1742140064),(92,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',19,200,'获取角色列表','',1742140065),(93,1,'吴俊','/adm/system/dictionary/list','GET','/system/dictionary/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取字典列表','',1742140066),(94,1,'吴俊','/adm/system/config/list','GET','/system/config/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',20,200,'获取系统配置项','',1742140068),(95,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',16,200,'操作日志列表','',1742140069),(96,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',5,200,'获取接口列表','',1742140070),(97,1,'吴俊','/adm/system/api/save','POST','/system/api/save','{\"id\": 1, \"path\": \"/adm/system/menu/save\", \"group\": \"系统管理\", \"method\": \"POST\", \"api_key\": \"system:menu:save\", \"description\": \"保存菜单\"}','{\"code\": 200, \"data\": {\"id\": 1, \"path\": \"/adm/system/menu/save\", \"group\": \"系统管理\", \"method\": \"POST\", \"api_key\": \"system:menu:save\", \"created_at\": 1739199895, \"updated_at\": 1742140073, \"description\": \"保存菜单\"}, \"message\": \"success\"}','','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',22,200,'保存接口','',1742140073),(98,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取接口列表','',1742140073),(99,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',22,200,'获取系统用户列表','',1742140075),(100,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=1000&sort_by=&sort_order=',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取角色列表','',1742140076),(101,1,'吴俊','/adm/system/user/save','POST','/system/user/save','{\"user\": {\"id\": 1, \"avatar\": \"\", \"status\": \"valid\", \"is_admin\": 0, \"password\": \"\", \"real_name\": \"吴俊\", \"user_name\": \"wujun\"}, \"role_ids\": [2]}','{\"code\": 200, \"data\": {\"id\": 1, \"avatar\": \"\", \"status\": \"valid\", \"is_admin\": 1, \"password\": \"d56d1bae0b780aef3b3355cc560e75df\", \"real_name\": \"吴俊\", \"user_name\": \"wujun\", \"created_at\": 0, \"updated_at\": 1742140078, \"is_init_pwd\": 1, \"last_update_password_time\": 1740323777}, \"message\": \"success\"}','','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',29,200,'保存用户','',1742140078),(102,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取系统用户列表','',1742140078),(103,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取角色列表','',1742140079),(104,1,'吴俊','/adm/system/role/save','POST','/system/role/save','{\"id\": 1, \"status\": \"valid\", \"role_name\": \"管理员\"}','{\"code\": 200, \"data\": {\"id\": 1, \"status\": \"valid\", \"role_name\": \"管理员\", \"created_at\": 1740326626, \"updated_at\": 1742140082}, \"message\": \"success\"}','','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',17,200,'保存角色','',1742140082),(105,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取角色列表','',1742140082),(106,1,'吴俊','/adm/system/dictionary/list','GET','/system/dictionary/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取字典列表','',1742140083),(107,1,'吴俊','/adm/system/dictionary/save','POST','/system/dictionary/save','{\"id\": 2, \"code\": \"user_level\", \"name\": \"用户层级\", \"remark\": \"用户分层，高低中三个等级\", \"status\": 1}','{\"code\": 200, \"data\": {\"id\": 2, \"code\": \"user_level\", \"name\": \"用户层级\", \"remark\": \"用户分层，高低中三个等级\", \"status\": 1, \"created_at\": 1741276194}, \"message\": \"success\"}','','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'保存字典信息','',1742140086),(108,1,'吴俊','/adm/system/dictionary/list','GET','/system/dictionary/list?page=1&page_size=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取字典列表','',1742140086),(109,1,'吴俊','/adm/system/config/list','GET','/system/config/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取系统配置项','',1742140087),(110,1,'吴俊','/adm/system/config/save','POST','/system/config/save','{\"id\": 1, \"key\": \"max_show_num\", \"name\": \"最大展示次数\", \"value\": \"100\", \"remark\": \"APP中给客户最多展示次数1\"}','{\"code\": 200, \"data\": {\"id\": 1, \"key\": \"max_show_num\", \"name\": \"最大展示次数\", \"value\": \"100\", \"remark\": \"APP中给客户最多展示次数1\", \"created_at\": 1741530809}, \"message\": \"success\"}','','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',17,200,'保存系统配置项','',1742140089),(111,1,'吴俊','/adm/system/config/list','GET','/system/config/list?page=1&page_size=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取系统配置项','',1742140089),(112,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'操作日志列表','',1742140091),(113,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=6&page_size=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',24,200,'操作日志列表','',1742140095),(114,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'操作日志列表','',1742140099),(115,1,'吴俊','/adm/system/config/list','GET','/system/config/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取系统配置项','',1742140103),(116,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取菜单列表','',1742140104),(117,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',3706,200,'获取用户分配的菜单列表','',1742215958),(118,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取菜单列表','',1742215962),(119,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取用户分配的菜单列表','',1742216464),(120,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取菜单列表','',1742216464),(121,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取菜单列表','',1742218207),(122,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',16,200,'获取接口列表','',1742218209),(123,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',16,200,'获取系统用户列表','',1742218210),(124,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取角色列表','',1742218211),(125,1,'吴俊','/adm/system/dictionary/list','GET','/system/dictionary/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',13,200,'获取字典列表','',1742218212),(126,1,'吴俊','/adm/system/config/list','GET','/system/config/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',31,200,'获取系统配置项','',1742218213),(127,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',27,200,'操作日志列表','',1742218214),(128,1,'吴俊','/adm/system/config/list','GET','/system/config/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',5,200,'获取系统配置项','',1742218218),(129,1,'吴俊','/adm/system/log/list','GET','/system/log/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',20,200,'操作日志列表','',1742218219),(130,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',12,200,'获取系统用户列表','',1742218999),(131,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',19,200,'获取接口列表','',1742219000),(132,1,'吴俊','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取菜单列表','',1742219000),(133,1,'吴俊','/adm/system/api/list','GET','/system/api/list?page=1&page_size=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取接口列表','',1742219007),(134,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',15,200,'获取角色列表','',1742219008),(135,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',19,200,'获取系统用户列表','',1742219010),(136,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=1000&sort_by=&sort_order=',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取角色列表','',1742219014),(137,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=1000&sort_by=&sort_order=',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',26,200,'获取角色列表','',1742219072),(138,1,'吴俊','/adm/system/user/save','POST','/system/user/save','{\"user\": {\"id\": 2, \"avatar\": \"\", \"status\": \"valid\", \"is_admin\": 0, \"password\": \"\", \"real_name\": \"周润发\", \"user_name\": \"zhourunfa\"}, \"role_ids\": []}','{\"code\": 200, \"data\": {\"id\": 2, \"avatar\": \"\", \"status\": \"valid\", \"is_admin\": 0, \"password\": \"d56d1bae0b780aef3b3355cc560e75df\", \"real_name\": \"周润发\", \"user_name\": \"zhourunfa\", \"created_at\": 1740324522, \"updated_at\": 1742219077, \"is_init_pwd\": 1, \"last_update_password_time\": 1741100917}, \"message\": \"success\"}','','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',38,200,'保存用户','',1742219077),(139,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',15,200,'获取系统用户列表','',1742219077),(140,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',13,200,'获取登录用户信息','',1742222284),(141,1,'吴俊','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,500,'未知操作','',1742222284),(142,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取登录用户信息','',1742222306),(143,1,'吴俊','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',83,500,'未知操作','',1742222306),(144,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取登录用户信息','',1742222310),(145,1,'吴俊','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,500,'未知操作','',1742222310),(146,1,'吴俊','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取登录用户信息','',1742222329),(147,1,'吴俊','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',93,500,'未知操作','',1742222330),(148,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',14,200,'获取系统用户列表','',1742225390),(149,1,'吴俊','/adm/system/role/list','GET','/system/role/list?page=1&pageSize=1000&sort_by=&sort_order=',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',12,200,'获取角色列表','',1742225398),(150,1,'吴俊','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',15,200,'获取用户分配的菜单列表','',1742225438),(151,1,'吴俊','/adm/system/user/list','GET','/system/user/list?page=1&pageSize=20&sort_by=name&sort_order=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',22,200,'获取系统用户列表','',1742225438),(152,1,'管理员','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取登录用户信息','',1742225449),(153,1,'管理员','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',14,200,'未知操作','',1742225449),(154,1,'管理员','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',16,200,'获取用户分配的菜单列表','',1742225449),(155,1,'管理员','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取用户分配的菜单列表','',1742225456),(156,1,'管理员','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',22,200,'获取用户分配的菜单列表','',1742225460),(157,1,'管理员','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',10,200,'获取菜单列表','',1742225460),(158,1,'管理员','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=100&sortBy=id&sortOrder=asc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取菜单列表','',1742225467),(159,1,'管理员','/adm/system/menu/save','POST','/system/menu/save','{\"id\": 9, \"link\": \"\", \"meta\": {\"icon\": \"ant-design:usergroup-add-outlined\", \"title\": \"首页\", \"hideInTab\": true, \"keepAlive\": true, \"hideInMenu\": true}, \"name\": \"user_info_manage\", \"path\": \"/home\", \"title\": \"首页\", \"children\": [], \"component\": \"/home/index\", \"parent_id\": 0}','{\"code\": 200, \"data\": {\"id\": 9, \"link\": \"\", \"meta\": {\"icon\": \"ant-design:usergroup-add-outlined\", \"title\": \"首页\", \"hideInTab\": true, \"keepAlive\": true, \"hideInMenu\": true}, \"name\": \"user_info_manage\", \"path\": \"/home\", \"title\": \"首页\", \"component\": \"/home/index\", \"parent_id\": 0, \"created_at\": 1741531448, \"updated_at\": 1742225488}, \"message\": \"success\"}','','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',19,200,'保存菜单','',1742225488),(160,1,'管理员','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',8,200,'获取菜单列表','',1742225488),(161,1,'管理员','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',6,200,'获取用户分配的菜单列表','',1742225492),(162,1,'管理员','/adm/system/menu/list','GET','/system/menu/list?page=1&pageSize=20&sortBy=name&sortOrder=desc',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',7,200,'获取菜单列表','',1742225492),(163,1,'管理员','/adm/system/access/user_info','GET','/system/access/user_info',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',9,200,'获取登录用户信息','',1742225515),(164,1,'管理员','/adm/system/access/codes','GET','/system/access/codes',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',88,200,'未知操作','',1742225515),(165,1,'管理员','/adm/system/menu/access_list','GET','/system/menu/access_list',NULL,NULL,'','Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36',11,200,'获取用户分配的菜单列表','',1742225516);
/*!40000 ALTER TABLE `system_admin_log` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_api`
--

DROP TABLE IF EXISTS `system_api`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_api` (
  `id` int NOT NULL AUTO_INCREMENT,
  `path` varchar(64) NOT NULL DEFAULT '' COMMENT '路径',
  `description` varchar(32) NOT NULL DEFAULT '' COMMENT '描述',
  `method` varchar(16) NOT NULL DEFAULT '' COMMENT '方法',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  `updated_at` int NOT NULL DEFAULT '0' COMMENT '更新时间',
  `api_key` varchar(64) NOT NULL DEFAULT '' COMMENT '权限标识',
  `group` varchar(32) NOT NULL DEFAULT '' COMMENT '分组',
  PRIMARY KEY (`id`),
  UNIQUE KEY `system_api_api_key_IDX` (`api_key`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=25 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_api`
--

LOCK TABLES `system_api` WRITE;
/*!40000 ALTER TABLE `system_api` DISABLE KEYS */;
INSERT INTO `system_api` VALUES (1,'/adm/system/menu/save','保存菜单','POST',1739199895,1742140073,'system:menu:save','系统管理'),(2,'/adm/system/menu/list','获取菜单列表','GET',1739285992,1741620043,'system:menu:list','系统管理'),(3,'/adm/system/menu/access_list','获取用户分配的菜单列表','GET',1741614693,1741616630,'system:access:menu_list','系统管理'),(4,'/adm/system/api/list','获取接口列表','GET',1741614929,1741616649,'system:api:list','系统管理'),(5,'/adm/system/user/list','获取系统用户列表','GET',1741614981,1741616675,'system:user:list','系统管理'),(6,'/adm/system/user/save','保存用户','POST',1741615019,1741616695,'system:user:save','系统管理'),(7,'/adm/system/api/save','保存接口','POST',1741615057,1741616707,'system:api:save','系统管理'),(8,'/adm/system/role/list','获取角色列表','GET',1741615657,1741616726,'system:role:list','系统管理'),(9,'/adm/system/role/menu_list','获取角色分配的菜单列表','GET',1741615779,1741616779,'system:role:menu_list','系统管理'),(10,'/adm/system/role/save_menu','保存角色分配的菜单','POST',1741615817,1741616807,'system:role:save_menu','系统管理'),(11,'/adm/system/role/api_list','获取角色分配的api接口','GET',1741615911,1741617246,'system:role:api_list','系统管理'),(12,'/adm/system/role/save_api','保存角色分配的接口','POST',1741615945,1741618987,'system:role:save_api','系统管理'),(13,'/adm//system/access/codes','获取登录用户的接口权限码','GET',1741616035,1741617340,'system:access:codes','系统管理'),(14,'/adm/system/role/save','保存角色','POST',1741616137,1741617408,'system:role:save','系统管理'),(15,'/adm/system/user/info/{id}','获取用户信息','GET',1741616190,1741617466,'system:user:info','系统管理'),(16,'/adm/system/access/user_info','获取登录用户信息','GET',1741616248,1741617491,'system:access:user_info','系统管理'),(17,'/adm/system/dictionary/list','获取字典列表','GET',1741616283,1741617514,'system:dictionary:list','系统管理'),(18,'/adm/system/dictionary/save','保存字典信息','POST',1741616314,1741617534,'system:dictionary:save','系统管理'),(19,'/adm/system/dictionary/config_list','获取字典配置项','GET',1741616355,1741617560,'system:dictionary:config_list','系统管理'),(20,'/adm/system/dictionary/save_config','保存字典配置项','POST',1741616394,1741617583,'system:dictionary:save_config','系统管理'),(21,'/adm/system/dictionary/delete/{id}','删除系统配置项','DELETE',1741616434,1741617612,'system:dictinary:delete_config','系统管理'),(22,'/adm/system/config/list','获取系统配置项','GET',1741616478,1741617628,'system:config:list','系统管理'),(23,'/adm/system/config/save','保存系统配置项','POST',1741616544,1741617637,'system:config:save','系统管理'),(24,'/adm/system/log/list','操作日志列表','GET',1742131376,1742131468,'system:log:list','系统管理');
/*!40000 ALTER TABLE `system_api` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_config`
--

DROP TABLE IF EXISTS `system_config`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_config` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '系统配置表',
  `name` varchar(64) NOT NULL DEFAULT '' COMMENT '系统名称',
  `key` varchar(64) NOT NULL DEFAULT '' COMMENT '参数key值',
  `value` varchar(512) NOT NULL DEFAULT '' COMMENT '参数值',
  `remark` varchar(512) NOT NULL DEFAULT '' COMMENT '备注信息',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `system_config_unique` (`name`),
  UNIQUE KEY `system_config_unique_1` (`key`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='系统配置表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_config`
--

LOCK TABLES `system_config` WRITE;
/*!40000 ALTER TABLE `system_config` DISABLE KEYS */;
INSERT INTO `system_config` VALUES (1,'最大展示次数','max_show_num','100','APP中给客户最多展示次数1',1741530809);
/*!40000 ALTER TABLE `system_config` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_data_dictionary`
--

DROP TABLE IF EXISTS `system_data_dictionary`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_data_dictionary` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `status` tinyint NOT NULL DEFAULT '1' COMMENT '状态0停用，1启用',
  `name` varchar(64) NOT NULL DEFAULT '' COMMENT '字典名称',
  `remark` varchar(256) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '备注信息',
  `code` varchar(64) CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci NOT NULL DEFAULT '' COMMENT '字典编码',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `system_data_dictionary_unique` (`name`),
  UNIQUE KEY `system_data_dictionary_unique_1` (`code`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='数据字典';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_data_dictionary`
--

LOCK TABLES `system_data_dictionary` WRITE;
/*!40000 ALTER TABLE `system_data_dictionary` DISABLE KEYS */;
INSERT INTO `system_data_dictionary` VALUES (2,1,'用户层级','用户分层，高低中三个等级','user_level',1741276194);
/*!40000 ALTER TABLE `system_data_dictionary` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_data_dictionary_config`
--

DROP TABLE IF EXISTS `system_data_dictionary_config`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_data_dictionary_config` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '自增ID',
  `data_dictionary_id` int NOT NULL DEFAULT '0' COMMENT '数据字典ID system_data_dictionary_id 的id',
  `label` varchar(128) NOT NULL DEFAULT '' COMMENT '配置项文案',
  `value` varchar(128) NOT NULL DEFAULT '' COMMENT '配置项的值',
  `remark` varchar(256) NOT NULL DEFAULT '' COMMENT '备注信息',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  `status` tinyint NOT NULL DEFAULT '1' COMMENT '状态1.启用，0停用',
  `is_del` tinyint NOT NULL DEFAULT '0' COMMENT '是否删除1.删除，0未删除',
  `sort` int NOT NULL DEFAULT '1' COMMENT '排序',
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='数据字典配置表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_data_dictionary_config`
--

LOCK TABLES `system_data_dictionary_config` WRITE;
/*!40000 ALTER TABLE `system_data_dictionary_config` DISABLE KEYS */;
INSERT INTO `system_data_dictionary_config` VALUES (1,2,'test','123','ddddd',1741512070,1,1,12),(2,2,'test','123','2333',1741512339,1,1,12),(3,2,'高冲','high','高冲用户',1741512369,1,0,12);
/*!40000 ALTER TABLE `system_data_dictionary_config` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_menu`
--

DROP TABLE IF EXISTS `system_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_menu` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `name` varchar(32) NOT NULL DEFAULT '' COMMENT '菜单唯一名称',
  `title` varchar(32) NOT NULL DEFAULT '' COMMENT '菜单名称',
  `link` varchar(255) NOT NULL DEFAULT '' COMMENT '链接地址',
  `path` varchar(64) NOT NULL DEFAULT '' COMMENT '路径',
  `meta` json DEFAULT NULL COMMENT 'meta数据',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  `updated_at` int NOT NULL DEFAULT '0' COMMENT '更新时间',
  `title_i18n` varchar(64) NOT NULL DEFAULT '' COMMENT '标题多语言配置',
  `parent_id` int NOT NULL DEFAULT '0' COMMENT '父菜单id',
  `component` varchar(128) NOT NULL DEFAULT '' COMMENT '组件',
  PRIMARY KEY (`id`),
  UNIQUE KEY `system_memu_unique` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_menu`
--

LOCK TABLES `system_menu` WRITE;
/*!40000 ALTER TABLE `system_menu` DISABLE KEYS */;
INSERT INTO `system_menu` VALUES (1,'system_manage','系统管理','','/system','{\"icon\": \"ant-design:menu-outlined\", \"title\": \"系统管理\", \"openInNewWindow\": true}',1739111270,1741619177,'',0,'/system/menus/index'),(2,'system_menus','菜单管理','','/menus','{\"icon\": \"ant-design:appstore-filled\", \"title\": \"菜单管理\"}',1739111270,1739195344,'',1,'/system/menus/index'),(4,'system_api','Api管理','','/apis','{\"icon\": \"ant-design:api-filled\", \"title\": \"Api管理\", \"openInNewWindow\": true}',1739113480,1739195403,'',1,'/system/apis/index'),(5,'system_user','用户管理','','/users','{\"icon\": \"ant-design:node-expand-outlined\", \"title\": \"用户管理\", \"keepAlive\": true}',1739978323,1741101851,'',1,'/system/user/index'),(6,'system_roles','角色管理','','/roles','{\"icon\": \"ant-design:discord-outlined\", \"title\": \"角色管理\", \"keepAlive\": true}',1740325887,1740325953,'',1,'/system/roles/index'),(7,'system_data_dictionary','数据字典','','/data-dictionary','{\"icon\": \"ant-design:insert-row-below-outlined\", \"title\": \"数据字典\", \"keepAlive\": true}',1741267406,1741274565,'',1,'/system/data-dictionary/index'),(8,'system_config','参数配置','','/config','{\"icon\": \"ant-design:setting-outlined\", \"title\": \"参数配置\", \"keepAlive\": true}',1741527356,1741531064,'',1,'/system/config/index'),(9,'user_info_manage','首页','','/home','{\"icon\": \"ant-design:usergroup-add-outlined\", \"title\": \"首页\", \"hideInTab\": true, \"keepAlive\": true, \"hideInMenu\": true}',1741531448,1742225488,'',0,'/home/index'),(10,'system_admin_log','操作日志','','/system/log/list','{\"icon\": \"ant-design:container-twotone\", \"title\": \"操作日志\", \"keepAlive\": true}',1742131328,1742131328,'',1,'/system/log/index');
/*!40000 ALTER TABLE `system_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_role`
--

DROP TABLE IF EXISTS `system_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_role` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '角色id',
  `role_name` varchar(32) NOT NULL DEFAULT '' COMMENT '角色名称',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '角色创建时间',
  `updated_at` int NOT NULL DEFAULT '0' COMMENT '更新时间',
  `status` varchar(16) NOT NULL DEFAULT 'valid' COMMENT '状态',
  PRIMARY KEY (`id`),
  UNIQUE KEY `system_role_unique` (`role_name`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='角色表';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_role`
--

LOCK TABLES `system_role` WRITE;
/*!40000 ALTER TABLE `system_role` DISABLE KEYS */;
INSERT INTO `system_role` VALUES (1,'管理员',1740326626,1742140082,'valid'),(2,'运营',1740327085,1741619113,'valid');
/*!40000 ALTER TABLE `system_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_role_api`
--

DROP TABLE IF EXISTS `system_role_api`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_role_api` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `system_api_id` int NOT NULL DEFAULT '0' COMMENT '系统接口id',
  `system_role_id` int NOT NULL DEFAULT '0' COMMENT '系统角色id',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `system_role_api_system_api_id_IDX` (`system_api_id`,`system_role_id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=50 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_role_api`
--

LOCK TABLES `system_role_api` WRITE;
/*!40000 ALTER TABLE `system_role_api` DISABLE KEYS */;
INSERT INTO `system_role_api` VALUES (26,8,2,1741619127),(27,9,2,1741619127),(28,10,2,1741619127),(29,11,2,1741619127),(30,12,2,1741619127),(31,13,2,1741619127),(32,14,2,1741619127),(33,15,2,1741619127),(34,16,2,1741619127),(35,17,2,1741619127),(36,18,2,1741619127),(37,19,2,1741619127),(38,20,2,1741619127),(39,21,2,1741619127),(40,22,2,1741619127),(41,23,2,1741619127),(42,1,2,1741698827),(43,2,2,1741698827),(44,3,2,1741698827),(46,5,2,1741698827),(47,6,2,1741698827),(48,7,2,1741698827),(49,4,2,1741705782);
/*!40000 ALTER TABLE `system_role_api` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_role_menu`
--

DROP TABLE IF EXISTS `system_role_menu`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_role_menu` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '自增主键',
  `system_menu_id` int NOT NULL DEFAULT '0' COMMENT '系统菜单id',
  `system_role_id` int NOT NULL DEFAULT '0' COMMENT '系统角色id',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  PRIMARY KEY (`id`),
  UNIQUE KEY `system_role_menu_system_menu_id_IDX` (`system_menu_id`,`system_role_id`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=31 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_role_menu`
--

LOCK TABLES `system_role_menu` WRITE;
/*!40000 ALTER TABLE `system_role_menu` DISABLE KEYS */;
INSERT INTO `system_role_menu` VALUES (16,5,1,1741098950),(19,5,2,1741098954),(23,4,2,1741101770),(24,2,1,1741137669),(25,4,1,1741137669),(26,6,1,1741137669),(27,1,1,1741137669),(28,2,2,1741137719),(29,6,2,1741137719),(30,1,2,1741137719);
/*!40000 ALTER TABLE `system_role_menu` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_user`
--

DROP TABLE IF EXISTS `system_user`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_user` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '自增',
  `real_name` varchar(32) NOT NULL DEFAULT '' COMMENT '真实姓名',
  `email` varchar(32) NOT NULL DEFAULT '' COMMENT '邮箱',
  `user_name` varchar(32) NOT NULL COMMENT '登录用户名',
  `password` varchar(64) NOT NULL DEFAULT '' COMMENT '登录密码',
  `status` varchar(16) NOT NULL DEFAULT 'valid' COMMENT '状态valid有效，invalid无效',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  `updated_at` int DEFAULT '0' COMMENT '更新时间',
  `is_init_pwd` tinyint NOT NULL DEFAULT '1' COMMENT '是否初始化密码0.不是初始化，1是初始密码',
  `last_update_password_time` int NOT NULL DEFAULT '0' COMMENT '最后更新密码时间',
  `avatar` varchar(128) NOT NULL DEFAULT '' COMMENT '头像',
  `is_admin` tinyint NOT NULL DEFAULT '0' COMMENT '是否管理员',
  PRIMARY KEY (`id`),
  UNIQUE KEY `system_user_unique` (`user_name`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='系统用户';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_user`
--

LOCK TABLES `system_user` WRITE;
/*!40000 ALTER TABLE `system_user` DISABLE KEYS */;
INSERT INTO `system_user` VALUES (1,'管理员','','admin','d56d1bae0b780aef3b3355cc560e75df','valid',0,1742140078,1,1740323777,'',1);
/*!40000 ALTER TABLE `system_user` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `system_user_role`
--

DROP TABLE IF EXISTS `system_user_role`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `system_user_role` (
  `id` int NOT NULL AUTO_INCREMENT COMMENT '自增组件',
  `system_user_id` int NOT NULL DEFAULT '0' COMMENT '用户id',
  `system_role_id` int NOT NULL DEFAULT '0' COMMENT '角色id',
  `created_at` int NOT NULL DEFAULT '0' COMMENT '创建时间',
  PRIMARY KEY (`id`),
  KEY `fk_user_role_user` (`system_user_id`),
  KEY `fk_user_role_user_role` (`system_role_id`)
) ENGINE=InnoDB AUTO_INCREMENT=25 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci COMMENT='系统用户角色';
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `system_user_role`
--

LOCK TABLES `system_user_role` WRITE;
/*!40000 ALTER TABLE `system_user_role` DISABLE KEYS */;
/*!40000 ALTER TABLE `system_user_role` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Dumping routines for database 'rust-admin'
--
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-03-17 23:32:14
