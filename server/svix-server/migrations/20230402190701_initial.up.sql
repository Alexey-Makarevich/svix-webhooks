-- svix1.application definition

CREATE TABLE `application` (
                               `id` varchar(255) NOT NULL,
                               `created_at` datetime(3) NOT NULL,
                               `updated_at` datetime(3) NOT NULL,
                               `org_id` varchar(255) NOT NULL,
                               `uid` varchar(255) DEFAULT NULL,
                               `name` varchar(255) NOT NULL,
                               `rate_limit` int DEFAULT NULL,
                               `deleted` tinyint(1) NOT NULL,
                               PRIMARY KEY (`id`),
                               UNIQUE KEY `ix_application_unique_uid_per_org_cond` (`org_id`,`uid`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- svix1.applicationmetadata definition

CREATE TABLE `applicationmetadata` (
                                       `created_at` datetime(3) NOT NULL,
                                       `updated_at` datetime(3) NOT NULL,
                                       `id` varchar(255) NOT NULL,
                                       `data` json NOT NULL,
                                       PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- svix1.endpointmetadata definition

CREATE TABLE `endpointmetadata` (
                                    `created_at` datetime(3) NOT NULL,
                                    `updated_at` datetime(3) NOT NULL,
                                    `id` varchar(255) NOT NULL,
                                    `data` json NOT NULL,
                                    PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- svix1.eventtype definition

CREATE TABLE `eventtype` (
                             `created_at` datetime(3) NOT NULL,
                             `updated_at` datetime(3) NOT NULL,
                             `id` varchar(255) NOT NULL,
                             `org_id` varchar(255) NOT NULL,
                             `description` varchar(255) NOT NULL,
                             `deleted` tinyint(1) NOT NULL,
                             `schemas` json DEFAULT NULL,
                             `name` varchar(255) NOT NULL,
                             `feature_flag` text,
                             PRIMARY KEY (`id`),
                             UNIQUE KEY `ix_event_type_unique_org` (`org_id`,`name`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- svix1.endpoint definition

CREATE TABLE `endpoint` (
                            `id` varchar(255) NOT NULL,
                            `created_at` datetime(3) NOT NULL,
                            `updated_at` datetime(3) NOT NULL,
                            `app_id` varchar(255) NOT NULL,
                            `key` longblob NOT NULL,
                            `url` text NOT NULL,
                            `description` varchar(255) NOT NULL,
                            `event_types_ids` json DEFAULT NULL,
                            `version` int NOT NULL,
                            `rate_limit` int DEFAULT NULL,
                            `deleted` tinyint(1) NOT NULL,
                            `disabled` tinyint(1) NOT NULL,
                            `first_failure_at` datetime(3) DEFAULT NULL,
                            `uid` varchar(255) DEFAULT NULL,
                            `old_keys` json DEFAULT NULL,
                            `channels` json DEFAULT NULL,
                            `headers` json DEFAULT NULL,
                            PRIMARY KEY (`id`),
                            UNIQUE KEY `ix_endpoint_uid_unique_app_cond` (`app_id`,`uid`),
                            CONSTRAINT `fk_endpoint_app_id_application` FOREIGN KEY (`app_id`) REFERENCES `application` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- svix1.message definition

CREATE TABLE `message` (
                           `id` varchar(255) NOT NULL,
                           `created_at` datetime(3) NOT NULL,
                           `org_id` varchar(255) NOT NULL,
                           `app_id` varchar(255) NOT NULL,
                           `event_type` varchar(255) NOT NULL,
                           `uid` varchar(255) DEFAULT NULL,
                           `payload` json DEFAULT NULL,
                           `channels` json DEFAULT NULL,
                           `expiration` datetime(3) NOT NULL DEFAULT ((now() + interval 90 day)),
                           PRIMARY KEY (`id`),
                           UNIQUE KEY `ix_message_uid_unique_app_cond` (`app_id`,`uid`),
                           KEY `ix_message_per_app` (`app_id`,`id` DESC),
                           KEY `message_payload_not_null_pidx` (`expiration`),
                           CONSTRAINT `fk_message_app_id_application` FOREIGN KEY (`app_id`) REFERENCES `application` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- svix1.messagedestination definition

CREATE TABLE `messagedestination` (
                                      `id` varchar(255) NOT NULL,
                                      `created_at` datetime(3) NOT NULL,
                                      `updated_at` datetime(3) NOT NULL,
                                      `msg_id` varchar(255) NOT NULL,
                                      `endp_id` varchar(255) NOT NULL,
                                      `status` smallint NOT NULL,
                                      `next_attempt` datetime(3) DEFAULT NULL,
                                      PRIMARY KEY (`id`),
                                      KEY `ix_messagedestination_per_endp_no_status` (`endp_id`,`id` DESC),
                                      KEY `ix_messagedestination_per_endp_with_status` (`endp_id`,`status`,`id` DESC),
                                      KEY `ix_messagedestination_per_msg_no_status` (`msg_id`),
                                      CONSTRAINT `fk_messagedestination_endp_id_endpoint` FOREIGN KEY (`endp_id`) REFERENCES `endpoint` (`id`) ON DELETE CASCADE,
                                      CONSTRAINT `fk_messagedestination_msg_id_message` FOREIGN KEY (`msg_id`) REFERENCES `message` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;


-- svix1.messageattempt definition

CREATE TABLE `messageattempt` (
                                  `id` varchar(255) NOT NULL,
                                  `created_at` datetime(3) NOT NULL,
                                  `msg_id` varchar(255) NOT NULL,
                                  `msg_dest_id` varchar(255) NOT NULL,
                                  `endp_id` varchar(255) NOT NULL,
                                  `url` text NOT NULL,
                                  `status` smallint NOT NULL,
                                  `response_status_code` smallint NOT NULL,
                                  `response` longtext NOT NULL,
                                  `ended_at` datetime(3) DEFAULT NULL,
                                  `trigger_type` smallint NOT NULL,
                                  PRIMARY KEY (`id`),
                                  KEY `fk_messageattempt_msg_dest_id_messagedestination` (`msg_dest_id`),
                                  KEY `ix_messageattempt_per_msg_no_status` (`msg_id`,`id` DESC),
                                  CONSTRAINT `fk_messageattempt_msg_dest_id_messagedestination` FOREIGN KEY (`msg_dest_id`) REFERENCES `messagedestination` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

