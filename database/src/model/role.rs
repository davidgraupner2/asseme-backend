use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// Role model that matches the SurrealDB table definition
///
/// Supports hierarchical role system with 5 levels:
/// 1. Super Admin - Full system access
/// 2. MSP Admin - MSP and customer management  
/// 3. Tenant Admin - Full tenant access
/// 4. User Manager - User management within tenant
/// 5. Standard User - Basic access
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    /// The unique identifier for the role (SurrealDB Thing)
    pub id: Option<Thing>,

    /// Role name (e.g., "Super Admin", "MSP Admin")
    pub name: String,

    /// Human-readable description of the role
    pub description: String,

    /// Hierarchy level (1-5, where 1 is highest)
    pub level: u8,

    /// List of permissions granted to this role
    pub permissions: Vec<String>,

    /// When the role was created
    pub created_at: Option<DateTime<Utc>>,
}

impl Role {
    /// Create a new role
    pub fn new(name: String, description: String, level: u8, permissions: Vec<String>) -> Self {
        Self {
            id: None,
            name,
            description,
            level,
            permissions,
            created_at: None, // Will be set by SurrealDB
        }
    }

    /// Check if this role has a specific permission
    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }

    /// Add a permission to this role
    pub fn add_permission(&mut self, permission: String) {
        if !self.permissions.contains(&permission) {
            self.permissions.push(permission);
        }
    }

    /// Remove a permission from this role
    pub fn remove_permission(&mut self, permission: &str) {
        self.permissions.retain(|p| p != permission);
    }

    /// Check if this role is higher level than another role
    pub fn is_higher_level_than(&self, other: &Role) -> bool {
        self.level < other.level // Lower number = higher level
    }

    /// Get all permissions as a formatted string
    pub fn permissions_display(&self) -> String {
        self.permissions.join(", ")
    }
}

/// Predefined role types for easier role management
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoleType {
    #[serde(rename = "super_admin")]
    SuperAdmin,
    #[serde(rename = "msp_admin")]
    MspAdmin,
    #[serde(rename = "tenant_admin")]
    TenantAdmin,
    #[serde(rename = "user_manager")]
    UserManager,
    #[serde(rename = "user")]
    User,
}

impl RoleType {
    /// Get the role name as it appears in the database
    pub fn as_str(&self) -> &'static str {
        match self {
            RoleType::SuperAdmin => "Super Admin",
            RoleType::MspAdmin => "MSP Admin",
            RoleType::TenantAdmin => "Tenant Admin",
            RoleType::UserManager => "User Manager",
            RoleType::User => "Standard User",
        }
    }

    /// Get the hierarchy level for this role type
    pub fn level(&self) -> u8 {
        match self {
            RoleType::SuperAdmin => 1,
            RoleType::MspAdmin => 2,
            RoleType::TenantAdmin => 3,
            RoleType::UserManager => 4,
            RoleType::User => 5,
        }
    }

    /// Get default permissions for this role type
    pub fn default_permissions(&self) -> Vec<String> {
        match self {
            RoleType::SuperAdmin => vec![
                "system.manage".to_string(),
                "tenant.create".to_string(),
                "tenant.delete".to_string(),
                "tenant.manage_all".to_string(),
                "tenant.view_all".to_string(),
                "user.create_all".to_string(),
                "user.update_all".to_string(),
                "user.delete_all".to_string(),
                "user.view_all".to_string(),
                "msp.create".to_string(),
                "msp.update_all".to_string(),
                "msp.delete_all".to_string(),
                "msp.view_all".to_string(),
                "billing.create_all".to_string(),
                "billing.update_all".to_string(),
                "billing.delete_all".to_string(),
                "billing.view_all".to_string(),
                "role.assign_all".to_string(),
                "role.view_all".to_string(),
                "super_admin.create".to_string(),
                "msp_admin.create".to_string(),
                "tenant_admin.create".to_string(),
                "token.view_all".to_string(),
                "token.create_all".to_string(),
                "token.update_all".to_string(),
                "token.delete_all".to_string(),
            ],
            RoleType::MspAdmin => vec![
                "tenant.view_own".to_string(),
                "tenant.update_own".to_string(),
                "tenant.view_customers".to_string(),
                "tenant.update_customers".to_string(),
                "user.create_customers".to_string(),
                "user.update_customers".to_string(),
                "user.view_customers".to_string(),
                "user.delete_customers".to_string(),
                "user.view_own_tenant".to_string(),
                "user.update_own_tenant".to_string(),
                "user.create_own_tenant".to_string(),
                "msp.view_own".to_string(),
                "msp.update_own".to_string(),
                "msp.manage_relationships".to_string(),
                "billing.view_customers".to_string(),
                "billing.update_customers".to_string(),
                "billing.create_customers".to_string(),
                "billing.view_own".to_string(),
                "billing.update_own".to_string(),
                "role.assign_customers".to_string(),
                "role.assign_own_tenant".to_string(),
            ],
            RoleType::TenantAdmin => vec![
                "tenant.view_own".to_string(),
                "tenant.update_own".to_string(),
                "tenant.manage_settings".to_string(),
                "user.create_own_tenant".to_string(),
                "user.update_own_tenant".to_string(),
                "user.view_own_tenant".to_string(),
                "user.delete_own_tenant".to_string(),
                "billing.view_own".to_string(),
                "billing.update_own".to_string(),
                "role.assign_own_tenant".to_string(),
                "role.view_own_tenant".to_string(),
            ],
            RoleType::UserManager => vec![
                "user.create_own_tenant".to_string(),
                "user.update_own_tenant".to_string(),
                "user.view_own_tenant".to_string(),
                "user.delete_basic_own_tenant".to_string(),
                "role.assign_basic_own_tenant".to_string(),
                "role.view_own_tenant".to_string(),
            ],
            RoleType::User => vec![
                "profile.edit".to_string(),
                "profile.view".to_string(),
                "data.view_own".to_string(),
                "settings.view".to_string(),
            ],
        }
    }
}
