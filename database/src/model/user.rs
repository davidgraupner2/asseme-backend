use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// User model that matches the SurrealDB table definition
///
/// Example usage:
/// ```rust
/// use database::model::user::User;
/// use surrealdb::sql::Thing;
///
/// // Create a new user
/// let primary_tenant = Thing::from(("tenant", "company_id"));
/// let user = User::new(
///     "john.doe@example.com".to_string(),
///     "John".to_string(),
///     "Doe".to_string(),
///     "hashed_password".to_string(),
///     primary_tenant
/// );
///
/// // Create a user that must change password on first login
/// let temp_user = User::new_with_temp_password(
///     "jane.doe@example.com".to_string(),
///     "Jane".to_string(),
///     "Doe".to_string(),
///     "temp_hash".to_string(),
///     Thing::from(("tenant", "company_id"))
/// );
/// ```

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// The unique identifier for the user (SurrealDB Thing)
    pub id: Option<Thing>,

    /// User's email address (must be valid email format, unique)
    pub email: String,

    /// User's first name
    pub first_name: String,

    /// User's last name
    pub last_name: String,

    /// Hashed password (only accessible for create/update and self-select)
    #[serde(skip_serializing)]
    pub password_hash: String,

    /// The primary tenant this user belongs to
    pub primary_tenant: Thing,

    /// List of tenants this user has access to
    #[serde(default)]
    pub accessible_tenants: Vec<Thing>,

    /// Whether the user account is active
    #[serde(default = "default_is_active")]
    pub is_active: bool,

    /// Optional phone number
    pub phone: Option<String>,

    /// When the user last logged in
    pub last_login: Option<DateTime<Utc>>,

    /// When the password was last changed
    pub password_changed_at: Option<DateTime<Utc>>,

    /// Whether the user must change their password on next login
    #[serde(default = "default_password_must_change")]
    pub password_must_change: bool,

    /// When the user record was created
    pub created_at: Option<DateTime<Utc>>,

    /// When the user record was last updated
    pub updated_at: Option<DateTime<Utc>>,
}

fn default_is_active() -> bool {
    true
}

fn default_password_must_change() -> bool {
    false
}

impl User {
    /// Create a new user with required fields
    pub fn new(
        email: String,
        first_name: String,
        last_name: String,
        password_hash: String,
        primary_tenant: Thing,
    ) -> Self {
        Self {
            id: None,
            email,
            first_name,
            last_name,
            password_hash,
            primary_tenant,
            accessible_tenants: Vec::new(),
            is_active: true,
            phone: None,
            last_login: None,
            password_changed_at: None,
            password_must_change: false,
            created_at: None, // Will be set by SurrealDB
            updated_at: None, // Will be set by SurrealDB
        }
    }

    /// Set the last_login to the current date and time in UTC
    pub fn set_last_login(&mut self) {
        self.last_login = Some(Utc::now());
    }

    /// Create a new user that must change password on first login
    pub fn new_with_temp_password(
        email: String,
        first_name: String,
        last_name: String,
        temp_password_hash: String,
        primary_tenant: Thing,
    ) -> Self {
        let mut user = Self::new(
            email,
            first_name,
            last_name,
            temp_password_hash,
            primary_tenant,
        );
        user.password_must_change = true;
        user
    }

    /// Get the user's full name
    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    /// Check if the user is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// Check if the user must change their password
    pub fn must_change_password(&self) -> bool {
        self.password_must_change
    }

    /// Check if the user has access to a specific tenant
    pub fn has_access_to_tenant(&self, tenant_id: &Thing) -> bool {
        &self.primary_tenant == tenant_id || self.accessible_tenants.contains(tenant_id)
    }

    /// Add access to a tenant
    pub fn add_tenant_access(&mut self, tenant_id: Thing) {
        if !self.accessible_tenants.contains(&tenant_id) && tenant_id != self.primary_tenant {
            self.accessible_tenants.push(tenant_id);
        }
    }

    /// Remove access to a tenant
    pub fn remove_tenant_access(&mut self, tenant_id: &Thing) {
        self.accessible_tenants.retain(|id| id != tenant_id);
    }

    /// Get all tenants the user has access to (including primary)
    pub fn all_accessible_tenants(&self) -> Vec<Thing> {
        let mut tenants = vec![self.primary_tenant.clone()];
        tenants.extend(self.accessible_tenants.clone());
        tenants
    }

    /// Deactivate the user account
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Activate the user account
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Mark that the user has logged in
    pub fn record_login(&mut self) {
        self.last_login = Some(Utc::now());
    }

    /// Update the password hash and mark when it was changed
    pub fn update_password(&mut self, new_password_hash: String) {
        self.password_hash = new_password_hash;
        self.password_changed_at = Some(Utc::now());
        self.password_must_change = false;
    }

    /// Force password change on next login
    pub fn require_password_change(&mut self) {
        self.password_must_change = true;
    }

    /// Check if the user's password is expired (based on last change time)
    pub fn is_password_expired(&self, max_age_days: i64) -> bool {
        match self.password_changed_at {
            Some(changed_at) => {
                let max_age = chrono::Duration::days(max_age_days);
                Utc::now() - changed_at > max_age
            }
            None => true, // No password change recorded means it's expired
        }
    }

    /// Update user's contact information
    pub fn update_contact_info(&mut self, phone: Option<String>) {
        self.phone = phone;
    }

    /// Update user's name
    pub fn update_name(&mut self, first_name: String, last_name: String) {
        self.first_name = first_name;
        self.last_name = last_name;
    }

    /// Validate email format (basic validation - SurrealDB will do the full validation)
    pub fn is_valid_email(email: &str) -> bool {
        email.contains('@') && email.contains('.') && !email.is_empty()
    }

    /// Update email (with basic validation)
    pub fn update_email(&mut self, email: String) -> Result<(), String> {
        if Self::is_valid_email(&email) {
            self.email = email;
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }
}
