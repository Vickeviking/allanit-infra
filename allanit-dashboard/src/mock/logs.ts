export type LogEntry = {
  id: number
  created_at: string
  expires_at: string
  level: 'INFO' | 'WARN' | 'ERROR' | 'DEBUG'
  module: 'Fetcher' | 'Ingestor' | 'DbWriter' | 'DeadLetter' | 'CommandBus' | 'Rocket' | 'EmailService' | 'AuthService' | 'PaymentService' | 'NotificationService'
  action: 'FETCH' | 'UPSERT' | 'REPROCESS' | 'ERROR' | 'SUCCESS' | 'RETRY' | 'TIMEOUT' | 'VALIDATION' | 'SEND' | 'RECEIVE' | 'PROCESS' | 'ARCHIVE'
  custom_msg: string
  details?: {
    duration?: number
    records_affected?: number
    error_code?: string
    retry_count?: number
    user_id?: string
    request_id?: string
  }
}

export const logs: LogEntry[] = [
  {
    id: 234,
    created_at: "2024-11-08T12:31:45Z",
    expires_at: "2024-12-08T12:31:45Z",
    level: "INFO",
    module: "DbWriter",
    action: "UPSERT_BATCH",
    custom_msg: "Inserted 42 customers in 512ms",
    details: {
      duration: 512,
      records_affected: 42,
      request_id: "req_abc123"
    }
  },
  {
    id: 235,
    created_at: "2024-11-08T12:30:22Z",
    expires_at: "2024-12-08T12:30:22Z",
    level: "WARN",
    module: "EmailService",
    action: "RETRY",
    custom_msg: "Email delivery failed, retrying in 5 minutes",
    details: {
      retry_count: 2,
      error_code: "SMTP_TIMEOUT",
      request_id: "req_def456"
    }
  },
  {
    id: 236,
    created_at: "2024-11-08T12:29:15Z",
    expires_at: "2024-12-08T12:29:15Z",
    level: "ERROR",
    module: "PaymentService",
    action: "ERROR",
    custom_msg: "Payment processing failed for invoice #2024-1145",
    details: {
      error_code: "PAYMENT_DECLINED",
      user_id: "user_789",
      request_id: "req_ghi789"
    }
  },
  {
    id: 237,
    created_at: "2024-11-08T12:28:33Z",
    expires_at: "2024-12-08T12:28:33Z",
    level: "INFO",
    module: "Fetcher",
    action: "FETCH",
    custom_msg: "Successfully fetched 156 email templates from API",
    details: {
      duration: 234,
      records_affected: 156,
      request_id: "req_jkl012"
    }
  },
  {
    id: 238,
    created_at: "2024-11-08T12:27:18Z",
    expires_at: "2024-12-08T12:27:18Z",
    level: "DEBUG",
    module: "AuthService",
    action: "VALIDATION",
    custom_msg: "Token validation successful for user admin",
    details: {
      duration: 45,
      user_id: "admin",
      request_id: "req_mno345"
    }
  },
  {
    id: 239,
    created_at: "2024-11-08T12:26:42Z",
    expires_at: "2024-12-08T12:26:42Z",
    level: "WARN",
    module: "Ingestor",
    action: "TIMEOUT",
    custom_msg: "Data ingestion timeout after 30 seconds",
    details: {
      duration: 30000,
      error_code: "INGESTION_TIMEOUT",
      request_id: "req_pqr678"
    }
  },
  {
    id: 240,
    created_at: "2024-11-08T12:25:56Z",
    expires_at: "2024-12-08T12:25:56Z",
    level: "INFO",
    module: "EmailService",
    action: "SEND",
    custom_msg: "Email campaign 'Vinternyhetsbrev 2024' sent to 247 recipients",
    details: {
      duration: 1200,
      records_affected: 247,
      request_id: "req_stu901"
    }
  },
  {
    id: 241,
    created_at: "2024-11-08T12:24:11Z",
    expires_at: "2024-12-08T12:24:11Z",
    level: "ERROR",
    module: "DbWriter",
    action: "ERROR",
    custom_msg: "Database connection lost during journal update",
    details: {
      error_code: "DB_CONNECTION_LOST",
      retry_count: 3,
      request_id: "req_vwx234"
    }
  },
  {
    id: 242,
    created_at: "2024-11-08T12:23:27Z",
    expires_at: "2024-12-08T12:23:27Z",
    level: "INFO",
    module: "NotificationService",
    action: "SEND",
    custom_msg: "Push notification sent to 89 mobile devices",
    details: {
      duration: 180,
      records_affected: 89,
      request_id: "req_yza567"
    }
  },
  {
    id: 243,
    created_at: "2024-11-08T12:22:44Z",
    expires_at: "2024-12-08T12:22:44Z",
    level: "WARN",
    module: "DeadLetter",
    action: "REPROCESS",
    custom_msg: "Reprocessing 12 failed email deliveries",
    details: {
      records_affected: 12,
      retry_count: 1,
      request_id: "req_bcd890"
    }
  },
  {
    id: 244,
    created_at: "2024-11-08T12:21:59Z",
    expires_at: "2024-12-08T12:21:59Z",
    level: "DEBUG",
    module: "CommandBus",
    action: "PROCESS",
    custom_msg: "Processing command: UpdateCustomerProfile",
    details: {
      duration: 67,
      user_id: "user_123",
      request_id: "req_efg123"
    }
  },
  {
    id: 245,
    created_at: "2024-11-08T12:21:13Z",
    expires_at: "2024-12-08T12:21:13Z",
    level: "INFO",
    module: "Rocket",
    action: "SUCCESS",
    custom_msg: "RocketMQ message consumed successfully",
    details: {
      duration: 23,
      request_id: "req_hij456"
    }
  },
  {
    id: 246,
    created_at: "2024-11-08T12:20:28Z",
    expires_at: "2024-12-08T12:20:28Z",
    level: "ERROR",
    module: "PaymentService",
    action: "ERROR",
    custom_msg: "Credit card validation failed for customer 456",
    details: {
      error_code: "CARD_VALIDATION_FAILED",
      user_id: "customer_456",
      request_id: "req_klm789"
    }
  },
  {
    id: 247,
    created_at: "2024-11-08T12:19:41Z",
    expires_at: "2024-12-08T12:19:41Z",
    level: "INFO",
    module: "DbWriter",
    action: "UPSERT",
    custom_msg: "Updated 3 journal entries with new tags",
    details: {
      duration: 89,
      records_affected: 3,
      request_id: "req_nop012"
    }
  },
  {
    id: 248,
    created_at: "2024-11-08T12:18:55Z",
    expires_at: "2024-12-08T12:18:55Z",
    level: "WARN",
    module: "EmailService",
    action: "RETRY",
    custom_msg: "SMTP server temporarily unavailable, retrying",
    details: {
      retry_count: 1,
      error_code: "SMTP_UNAVAILABLE",
      request_id: "req_qrs345"
    }
  },
  {
    id: 249,
    created_at: "2024-11-08T12:18:09Z",
    expires_at: "2024-12-08T12:18:09Z",
    level: "DEBUG",
    module: "AuthService",
    action: "VALIDATION",
    custom_msg: "JWT token expires in 15 minutes",
    details: {
      user_id: "user_789",
      request_id: "req_tuv678"
    }
  },
  {
    id: 250,
    created_at: "2024-11-08T12:17:22Z",
    expires_at: "2024-12-08T12:17:22Z",
    level: "INFO",
    module: "Fetcher",
    action: "FETCH",
    custom_msg: "Fetched 67 sent emails from external API",
    details: {
      duration: 156,
      records_affected: 67,
      request_id: "req_wxy901"
    }
  },
  {
    id: 251,
    created_at: "2024-11-08T12:16:38Z",
    expires_at: "2024-12-08T12:16:38Z",
    level: "ERROR",
    module: "Ingestor",
    action: "ERROR",
    custom_msg: "Failed to parse JSON response from subsidiary API",
    details: {
      error_code: "JSON_PARSE_ERROR",
      request_id: "req_zab234"
    }
  },
  {
    id: 252,
    created_at: "2024-11-08T12:15:51Z",
    expires_at: "2024-12-08T12:15:51Z",
    level: "INFO",
    module: "NotificationService",
    action: "ARCHIVE",
    custom_msg: "Archived 45 expired log entries",
    details: {
      duration: 234,
      records_affected: 45,
      request_id: "req_cde567"
    }
  },
  {
    id: 253,
    created_at: "2024-11-08T12:15:05Z",
    expires_at: "2024-12-08T12:15:05Z",
    level: "WARN",
    module: "DbWriter",
    action: "TIMEOUT",
    custom_msg: "Database query timeout after 10 seconds",
    details: {
      duration: 10000,
      error_code: "QUERY_TIMEOUT",
      request_id: "req_fgh890"
    }
  },
  {
    id: 254,
    created_at: "2024-11-08T12:14:18Z",
    expires_at: "2024-12-08T12:14:18Z",
    level: "INFO",
    module: "EmailService",
    action: "SEND",
    custom_msg: "Service reminder email sent to 156 customers",
    details: {
      duration: 890,
      records_affected: 156,
      request_id: "req_ijk123"
    }
  }
]

export default logs