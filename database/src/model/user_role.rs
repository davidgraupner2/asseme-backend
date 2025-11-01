use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// User-Role relationship model with tenant context
///
/// This table links users to roles within specific tenants,
/// supporting the multi-tenant permission system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    /// The unique identifier for the user-role assignment
    pub id: Option<Thing>,

    /// Reference to the user
    pub user: Thing,

    /// Reference to the role being assigned
    pub role: Thing,

    /// The tenant context for this role assignment
    pub tenant: Thing,

    /// Who granted this role assignment
    pub granted_by: Thing,

    /// When the role was granted
    pub granted_at: DateTime<Utc>,

    /// When this role assignment expires (optional)
    pub expires_at: Option<DateTime<Utc>>,

    /// Whether this role assignment is currently active
    #[serde(default = "default_is_active")]
    pub is_active: bool,
}

fn default_is_active() -> bool {
    true
}

impl UserRole {
    /// Create a new user-role assignment
    pub fn new(user: Thing, role: Thing, tenant: Thing, granted_by: Thing) -> Self {
        Self {
            id: None,
            user,
            role,
            tenant,
            granted_by,
            granted_at: chrono::Utc::now(),
            expires_at: None,
            is_active: true,
        }
    }

    /// Create a temporary role assignment with expiration
    pub fn new_temporary(
        user: Thing,
        role: Thing,
        tenant: Thing,
        granted_by: Thing,
        expires_at: DateTime<Utc>,
    ) -> Self {
        let mut user_role = Self::new(user, role, tenant, granted_by);
        user_role.expires_at = Some(expires_at);
        user_role
    }

    /// Check if this role assignment is currently valid
    pub fn is_valid(&self) -> bool {
        if !self.is_active {
            return false;
        }

        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() <= expires_at
        } else {
            true
        }
    }

    /// Check if this role assignment has expired
    pub fn is_expired(&self) -> bool {
        if let Some(expires_at) = self.expires_at {
            chrono::Utc::now() > expires_at
        } else {
            false
        }
    }

    /// Deactivate this role assignment
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Reactivate this role assignment
    pub fn reactivate(&mut self) {
        self.is_active = true;
    }

    /// Extend the expiration date
    pub fn extend_expiration(&mut self, new_expiration: DateTime<Utc>) {
        self.expires_at = Some(new_expiration);
    }

    /// Remove expiration (make permanent)
    pub fn make_permanent(&mut self) {
        self.expires_at = None;
    }

    /// Get time remaining before expiration
    pub fn time_remaining(&self) -> Option<chrono::Duration> {
        self.expires_at.map(|expires| expires - chrono::Utc::now())
    }

    /// Check if expiration is within a certain threshold
    pub fn expires_soon(&self, threshold: chrono::Duration) -> bool {
        if let Some(remaining) = self.time_remaining() {
            remaining <= threshold && remaining > chrono::Duration::zero()
        } else {
            false
        }
    }
}
