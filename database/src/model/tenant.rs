use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::sql::Thing;

/// Tenant model that matches the SurrealDB table definition
///
/// Example usage:
/// ```rust
/// use database::model::tenant::{Tenant, TenantType};
///
/// // Create a new customer tenant
/// let tenant = Tenant::new(
///     "Acme Corp".to_string(),
///     "acme-corp".to_string(),
///     "admin@acme.com".to_string()
/// );
///
/// // Create an MSP tenant
/// let msp = Tenant::new_msp(
///     "MSP Provider".to_string(),
///     "msp-provider".to_string(),
///     "contact@msp.com".to_string()
/// );
/// ```

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantType {
    #[serde(rename = "super_admin")]
    SuperAdmin,
    #[serde(rename = "msp")]
    Msp,
    #[serde(rename = "customer")]
    Customer,
}

impl Default for TenantType {
    fn default() -> Self {
        TenantType::Customer
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TenantStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Default for TenantStatus {
    fn default() -> Self {
        TenantStatus::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BillingResponsibility {
    #[serde(rename = "self")]
    SelfBilling,
    #[serde(rename = "msp")]
    Msp,
    #[serde(rename = "parent")]
    Parent,
}

impl Default for BillingResponsibility {
    fn default() -> Self {
        BillingResponsibility::SelfBilling
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    /// The unique identifier for the tenant (SurrealDB Thing)
    pub id: Option<Thing>,

    /// The display name of the tenant
    pub name: String,

    /// Unique slug identifier for the tenant
    pub slug: String,

    /// Type of tenant (super_admin, msp, customer)
    #[serde(default)]
    pub tenant_type: TenantType,

    /// Current status of the tenant
    #[serde(default)]
    pub status: TenantStatus,

    /// Contact email for the tenant
    pub contact_email: String,

    /// Optional contact phone number
    pub contact_phone: Option<String>,

    /// Whether billing is enabled for this tenant
    #[serde(default = "default_billing_enabled")]
    pub billing_enabled: bool,

    /// Who is responsible for billing
    #[serde(default)]
    pub billing_responsibility: BillingResponsibility,

    /// Reference to the MSP tenant (if applicable)
    pub msp_tenant: Option<Thing>,

    /// Reference to the parent tenant (if applicable)  
    pub parent_tenant: Option<Thing>,

    /// Additional settings stored as key-value pairs
    #[serde(default)]
    pub settings: HashMap<String, serde_json::Value>,

    /// When the tenant was created
    pub created_at: Option<DateTime<Utc>>,

    /// When the tenant was last updated
    pub updated_at: Option<DateTime<Utc>>,
}

fn default_billing_enabled() -> bool {
    true
}

impl Tenant {
    /// Create a new tenant with required fields
    pub fn new(name: String, slug: String, contact_email: String) -> Self {
        Self {
            id: None,
            name,
            slug,
            contact_email,
            tenant_type: TenantType::default(),
            status: TenantStatus::default(),
            contact_phone: None,
            billing_enabled: true,
            billing_responsibility: BillingResponsibility::default(),
            msp_tenant: None,
            parent_tenant: None,
            settings: HashMap::new(),
            created_at: None, // Will be set by SurrealDB
            updated_at: None, // Will be set by SurrealDB
        }
    }

    /// Create a new MSP tenant
    pub fn new_msp(name: String, slug: String, contact_email: String) -> Self {
        let mut tenant = Self::new(name, slug, contact_email);
        tenant.tenant_type = TenantType::Msp;
        tenant
    }

    /// Create a new customer tenant under an MSP
    pub fn new_customer(
        name: String,
        slug: String,
        contact_email: String,
        msp_tenant: Thing,
    ) -> Self {
        let mut tenant = Self::new(name, slug, contact_email);
        tenant.tenant_type = TenantType::Customer;
        tenant.msp_tenant = Some(msp_tenant);
        tenant
    }

    /// Check if this is a super admin tenant
    pub fn is_super_admin(&self) -> bool {
        matches!(self.tenant_type, TenantType::SuperAdmin)
    }

    /// Check if this is an MSP tenant
    pub fn is_msp(&self) -> bool {
        matches!(self.tenant_type, TenantType::Msp)
    }

    /// Check if this is a customer tenant
    pub fn is_customer(&self) -> bool {
        matches!(self.tenant_type, TenantType::Customer)
    }

    /// Check if the tenant is active
    pub fn is_active(&self) -> bool {
        matches!(self.status, TenantStatus::Active)
    }

    /// Check if billing is enabled
    pub fn billing_enabled(&self) -> bool {
        self.billing_enabled
    }

    /// Get the MSP tenant reference if this is a customer
    pub fn get_msp_tenant(&self) -> Option<&Thing> {
        self.msp_tenant.as_ref()
    }

    /// Get the parent tenant reference if applicable
    pub fn get_parent_tenant(&self) -> Option<&Thing> {
        self.parent_tenant.as_ref()
    }

    /// Add or update a setting
    pub fn set_setting(&mut self, key: String, value: serde_json::Value) {
        self.settings.insert(key, value);
    }

    /// Get a setting value
    pub fn get_setting(&self, key: &str) -> Option<&serde_json::Value> {
        self.settings.get(key)
    }

    /// Remove a setting
    pub fn remove_setting(&mut self, key: &str) -> Option<serde_json::Value> {
        self.settings.remove(key)
    }
}
