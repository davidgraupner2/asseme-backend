/// MSP Multi-Tenant Database Library
///
/// This library provides complete SurrealDB schema management and Rust models
/// for a multi-tenant MSP (Managed Service Provider) system.
///
/// ## Features
///
/// - **Complete Schema Management**: Automatic database initialization from SQL
/// - **Multi-Tenant Support**: Hierarchical tenant structure (Super Admin → MSP → Customer)
/// - **Role-Based Access Control**: 5-level permission system
/// - **MSP Billing**: Commission tracking and billing arrangements
/// - **User Management**: Full user lifecycle with tenant relationships
/// - **Token Management**: API access and agent authentication
///
/// ## Quick Start
///
/// ```rust
/// use database::context::get_initialized_database;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Get database with automatic schema initialization
///     let db = get_initialized_database(
///         "ws".to_string(),
///         "localhost:8000".to_string(),
///         "root".to_string(),
///         "root".to_string(),
///         "asseme".to_string(),
///         "msp".to_string(),
///         false, // Don't force reinit
///     ).await?;
///     
///     // Database is ready with all tables, roles, and sample data!
///     Ok(())
/// }
/// ```
///
/// ## Schema Overview
///
/// The schema supports a complete MSP hierarchy:
///
/// - **Tenants**: Super Admin, MSP, and Customer levels
/// - **Users**: Multi-tenant access with role assignments
/// - **Roles**: 5-level hierarchy with granular permissions
/// - **Billing**: MSP commission tracking and payment management
/// - **Relationships**: MSP-Customer contract management
/// - **Tokens**: API access and agent authentication
///
/// All models include comprehensive helper methods and type safety.
pub mod context;
pub mod model;
pub mod repository;
