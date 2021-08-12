# Showcase
A [Rust](https://www.rust-lang.org/) webservice using [Actix](https://actix.rs/).

# Prerequisites
- Rust 1.54+
- MySQL 8
- Crate tables against your database.
``` SQL
-- users definition
CREATE TABLE `users` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- posts definition
CREATE TABLE `posts` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `user_id` bigint unsigned NOT NULL,
  `title` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `content` text CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT NULL,
  `updated_at` timestamp NULL DEFAULT NULL,
  `is_read` tinyint(1) DEFAULT NULL,
  PRIMARY KEY (`id`),
  KEY `posts_users_user_id_foreign` (`user_id`),
  CONSTRAINT `posts_users_id_foreign` FOREIGN KEY (`user_id`) REFERENCES `users` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=0 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
```
- Create at least one user for posting.
```SQL
INSERT INTO users(name, created_at, updated_at) VALUES('harvey1234567', '2021-08-12 00:00:00', '2021-08-12 00:00:00');
```

# How to
## Build
```cargo build```
## Run
- Have your `.env file` reference `.env.mysql` for example.
```./target/debug/rust-practice```
## Run test script
```bash test-post.sh```.
