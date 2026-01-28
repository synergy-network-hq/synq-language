//! Audit Logging System for Aegis
//!
//! This module provides comprehensive audit logging for all cryptographic
//! operations and security events.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use zeroize::Zeroize;

/// Audit log entry
#[derive(Debug, Clone)]
pub struct AuditLogEntry {
    /// Log entry timestamp
    pub timestamp: SystemTime,
    /// Log entry ID
    pub entry_id: String,
    /// Event type
    pub event_type: String,
    /// Event category
    pub category: AuditCategory,
    /// Event severity
    pub severity: AuditSeverity,
    /// User ID (if applicable)
    pub user_id: Option<String>,
    /// Session ID (if applicable)
    pub session_id: Option<String>,
    /// Source component
    pub source: String,
    /// Event message
    pub message: String,
    /// Additional event data
    pub data: HashMap<String, String>,
    /// Cryptographic operation details (if applicable)
    pub crypto_details: Option<CryptoOperationDetails>,
    /// IP address (if applicable)
    pub ip_address: Option<String>,
    /// User agent (if applicable)
    pub user_agent: Option<String>,
    /// Success/failure status
    pub success: bool,
    /// Error message (if applicable)
    pub error_message: Option<String>,
}

/// Audit categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuditCategory {
    /// Authentication events
    Authentication,
    /// Authorization events
    Authorization,
    /// Cryptographic operations
    Cryptographic,
    /// Key management operations
    KeyManagement,
    /// System events
    System,
    /// Security events
    Security,
    /// Data access events
    DataAccess,
    /// Configuration changes
    Configuration,
    /// Administrative actions
    Administrative,
}

/// Audit severity levels
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AuditSeverity {
    /// Informational
    Info,
    /// Warning
    Warning,
    /// Error
    Error,
    /// Critical
    Critical,
}

/// Cryptographic operation details
#[derive(Debug, Clone)]
pub struct CryptoOperationDetails {
    /// Algorithm used
    pub algorithm: String,
    /// Key ID used
    pub key_id: String,
    /// Operation type
    pub operation_type: String,
    /// Input size (in bytes)
    pub input_size: Option<u64>,
    /// Output size (in bytes)
    pub output_size: Option<u64>,
    /// Processing time (in microseconds)
    pub processing_time: Option<u64>,
    /// Security level
    pub security_level: Option<u32>,
}

/// Audit logger configuration
#[derive(Debug, Clone)]
pub struct AuditLoggerConfig {
    /// Maximum number of log entries to keep in memory
    pub max_entries: usize,
    /// Log retention period
    pub retention_period: std::time::Duration,
    /// Enable real-time logging
    pub real_time_logging: bool,
    /// Enable file logging
    pub file_logging: bool,
    /// Log file path
    pub log_file_path: Option<String>,
    /// Enable remote logging
    pub remote_logging: bool,
    /// Remote logging endpoint
    pub remote_endpoint: Option<String>,
    /// Log level threshold
    pub log_level_threshold: AuditSeverity,
}

impl Default for AuditLoggerConfig {
    fn default() -> Self {
        Self {
            max_entries: 100000,
            retention_period: std::time::Duration::from_secs(86400 * 90), // 90 days
            real_time_logging: true,
            file_logging: false,
            log_file_path: None,
            remote_logging: false,
            remote_endpoint: None,
            log_level_threshold: AuditSeverity::Info,
        }
    }
}

/// Audit logger
pub struct AuditLogger {
    /// Log entries storage
    entries: Arc<RwLock<Vec<AuditLogEntry>>>,
    /// Configuration
    config: Arc<RwLock<AuditLoggerConfig>>,
    /// Event handlers
    event_handlers: Arc<RwLock<Vec<Box<dyn Fn(&AuditLogEntry) + Send + Sync>>>>,
    /// Statistics
    stats: Arc<Mutex<AuditStats>>,
}

/// Audit statistics
#[derive(Debug, Clone)]
pub struct AuditStats {
    /// Total log entries
    pub total_entries: u64,
    /// Entries by category
    pub entries_by_category: HashMap<AuditCategory, u64>,
    /// Entries by severity
    pub entries_by_severity: HashMap<AuditSeverity, u64>,
    /// Successful operations
    pub successful_operations: u64,
    /// Failed operations
    pub failed_operations: u64,
    /// Last updated timestamp
    pub last_updated: SystemTime,
}

impl Default for AuditStats {
    fn default() -> Self {
        Self {
            total_entries: 0,
            entries_by_category: HashMap::new(),
            entries_by_severity: HashMap::new(),
            successful_operations: 0,
            failed_operations: 0,
            last_updated: SystemTime::now(),
        }
    }
}

impl AuditLogger {
    /// Create a new audit logger
    pub fn new(config: AuditLoggerConfig) -> Self {
        Self {
            entries: Arc::new(RwLock::new(Vec::new())),
            config: Arc::new(RwLock::new(config)),
            event_handlers: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(Mutex::new(AuditStats::default())),
        }
    }

    /// Log an audit event
    pub fn log_event(&self, mut entry: AuditLogEntry) {
        // Generate entry ID if not provided
        if entry.entry_id.is_empty() {
            entry.entry_id = format!("audit_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos());
        }

        // Check if event meets log level threshold
        let config = self.config.read().unwrap();
        if self.should_log_event(&entry, &config) {
            // Store entry
            {
                let mut entries = self.entries.write().unwrap();
                entries.push(entry.clone());

                // Maintain entry limit
                if entries.len() > config.max_entries {
                    entries.drain(0..config.max_entries / 10);
                }
            }

            // Update statistics
            self.update_stats(&entry);

            // Trigger event handlers
            self.trigger_event_handlers(&entry);

            // Log to file if enabled
            if config.file_logging {
                self.log_to_file(&entry);
            }

            // Log to remote endpoint if enabled
            if config.remote_logging {
                self.log_to_remote(&entry);
            }
        }
    }

    /// Log cryptographic operation
    pub fn log_crypto_operation(
        &self,
        algorithm: String,
        key_id: String,
        operation_type: String,
        success: bool,
        error_message: Option<String>,
        additional_data: Option<HashMap<String, String>>,
    ) {
        let mut data = additional_data.unwrap_or_default();
        data.insert("algorithm".to_string(), algorithm.clone());
        data.insert("key_id".to_string(), key_id.clone());
        data.insert("operation_type".to_string(), operation_type.clone());

        let entry = AuditLogEntry {
            timestamp: SystemTime::now(),
            entry_id: String::new(),
            event_type: "crypto_operation".to_string(),
            category: AuditCategory::Cryptographic,
            severity: if success { AuditSeverity::Info } else { AuditSeverity::Error },
            user_id: None,
            session_id: None,
            source: "AegisCrypto".to_string(),
            message: format!("Cryptographic operation: {} using {}", operation_type, algorithm),
            data,
            crypto_details: Some(CryptoOperationDetails {
                algorithm,
                key_id,
                operation_type,
                input_size: None,
                output_size: None,
                processing_time: None,
                security_level: None,
            }),
            ip_address: None,
            user_agent: None,
            success,
            error_message,
        };

        self.log_event(entry);
    }

    /// Log key management operation
    pub fn log_key_operation(
        &self,
        key_id: String,
        operation_type: String,
        algorithm: String,
        success: bool,
        error_message: Option<String>,
    ) {
        let mut data = HashMap::new();
        data.insert("key_id".to_string(), key_id.clone());
        data.insert("operation_type".to_string(), operation_type.clone());
        data.insert("algorithm".to_string(), algorithm.clone());

        let entry = AuditLogEntry {
            timestamp: SystemTime::now(),
            entry_id: String::new(),
            event_type: "key_operation".to_string(),
            category: AuditCategory::KeyManagement,
            severity: if success { AuditSeverity::Info } else { AuditSeverity::Error },
            user_id: None,
            session_id: None,
            source: "AegisKeyManager".to_string(),
            message: format!("Key operation: {} on key {}", operation_type, key_id),
            data,
            crypto_details: None,
            ip_address: None,
            user_agent: None,
            success,
            error_message,
        };

        self.log_event(entry);
    }

    /// Log security event
    pub fn log_security_event(
        &self,
        event_type: String,
        severity: AuditSeverity,
        message: String,
        additional_data: Option<HashMap<String, String>>,
    ) {
        let entry = AuditLogEntry {
            timestamp: SystemTime::now(),
            entry_id: String::new(),
            event_type,
            category: AuditCategory::Security,
            severity,
            user_id: None,
            session_id: None,
            source: "AegisSecurity".to_string(),
            message,
            data: additional_data.unwrap_or_default(),
            crypto_details: None,
            ip_address: None,
            user_agent: None,
            success: true,
            error_message: None,
        };

        self.log_event(entry);
    }

    /// Add event handler
    pub fn add_event_handler<F>(&self, handler: F)
    where
        F: Fn(&AuditLogEntry) + Send + Sync + 'static,
    {
        let mut handlers = self.event_handlers.write().unwrap();
        handlers.push(Box::new(handler));
    }

    /// Get audit statistics
    pub fn get_stats(&self) -> AuditStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Get log entries by time range
    pub fn get_entries_by_time_range(&self, start: SystemTime, end: SystemTime) -> Vec<AuditLogEntry> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .filter(|entry| entry.timestamp >= start && entry.timestamp <= end)
            .cloned()
            .collect()
    }

    /// Get log entries by category
    pub fn get_entries_by_category(&self, category: AuditCategory) -> Vec<AuditLogEntry> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .filter(|entry| entry.category == category)
            .cloned()
            .collect()
    }

    /// Get log entries by severity
    pub fn get_entries_by_severity(&self, severity: AuditSeverity) -> Vec<AuditLogEntry> {
        let entries = self.entries.read().unwrap();
        entries
            .iter()
            .filter(|entry| entry.severity == severity)
            .cloned()
            .collect()
    }

    /// Clean up old log entries
    pub fn cleanup_old_entries(&self) {
        let config = self.config.read().unwrap();
        let cutoff_time = SystemTime::now() - config.retention_period;

        let mut entries = self.entries.write().unwrap();
        entries.retain(|entry| entry.timestamp > cutoff_time);
    }

    /// Check if event should be logged based on configuration
    fn should_log_event(&self, entry: &AuditLogEntry, config: &AuditLoggerConfig) -> bool {
        match (&entry.severity, &config.log_level_threshold) {
            (AuditSeverity::Critical, _) => true,
            (AuditSeverity::Error, AuditSeverity::Info | AuditSeverity::Warning | AuditSeverity::Error) => true,
            (AuditSeverity::Warning, AuditSeverity::Info | AuditSeverity::Warning) => true,
            (AuditSeverity::Info, AuditSeverity::Info) => true,
            _ => false,
        }
    }

    /// Update audit statistics
    fn update_stats(&self, entry: &AuditLogEntry) {
        let mut stats = self.stats.lock().unwrap();
        stats.total_entries += 1;
        stats.last_updated = SystemTime::now();

        // Update entries by category
        *stats.entries_by_category.entry(entry.category.clone()).or_insert(0) += 1;

        // Update entries by severity
        *stats.entries_by_severity.entry(entry.severity.clone()).or_insert(0) += 1;

        // Update success/failure counters
        if entry.success {
            stats.successful_operations += 1;
        } else {
            stats.failed_operations += 1;
        }
    }

    /// Trigger event handlers
    fn trigger_event_handlers(&self, entry: &AuditLogEntry) {
        let handlers = self.event_handlers.read().unwrap();
        for handler in handlers.iter() {
            handler(entry);
        }
    }

    /// Log to file
    fn log_to_file(&self, entry: &AuditLogEntry) {
        // Implementation would write to file
        // This is a placeholder for file logging functionality
    }

    /// Log to remote endpoint
    fn log_to_remote(&self, entry: &AuditLogEntry) {
        // Implementation would send to remote endpoint
        // This is a placeholder for remote logging functionality
    }
}

impl Default for AuditLogger {
    fn default() -> Self {
        Self::new(AuditLoggerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audit_logger_creation() {
        let config = AuditLoggerConfig::default();
        let logger = AuditLogger::new(config);

        let stats = logger.get_stats();
        assert_eq!(stats.total_entries, 0);
    }

    #[test]
    fn test_crypto_operation_logging() {
        let logger = AuditLogger::default();

        logger.log_crypto_operation(
            "ML-KEM-768".to_string(),
            "key_123".to_string(),
            "encapsulate".to_string(),
            true,
            None,
            None,
        );

        let stats = logger.get_stats();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.successful_operations, 1);
    }

    #[test]
    fn test_key_operation_logging() {
        let logger = AuditLogger::default();

        logger.log_key_operation(
            "key_456".to_string(),
            "generate".to_string(),
            "ML-DSA-65".to_string(),
            true,
            None,
        );

        let stats = logger.get_stats();
        assert_eq!(stats.total_entries, 1);
        assert_eq!(stats.successful_operations, 1);
    }

    #[test]
    fn test_security_event_logging() {
        let logger = AuditLogger::default();

        logger.log_security_event(
            "anomaly_detected".to_string(),
            AuditSeverity::Error,
            "Unusual key usage pattern detected".to_string(),
            None,
        );

        let stats = logger.get_stats();
        assert_eq!(stats.total_entries, 1);
    }
}
