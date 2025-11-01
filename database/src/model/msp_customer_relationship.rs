use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// MSP-Customer relationship tracking model
///
/// This table manages the relationships between MSP tenants and their customers,
/// including billing arrangements and contract details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MspCustomerRelationship {
    /// The unique identifier for the relationship
    pub id: Option<Thing>,

    /// Reference to the MSP tenant
    pub msp_tenant: Thing,

    /// Reference to the customer tenant
    pub customer_tenant: Thing,

    /// Type of relationship between MSP and customer
    #[serde(default)]
    pub relationship_type: RelationshipType,

    /// How billing is arranged between MSP and customer
    #[serde(default)]
    pub billing_arrangement: BillingArrangement,

    /// Commission rate for the MSP (optional, as percentage)
    pub commission_rate: Option<f64>,

    /// When the contract started
    pub contract_start: DateTime<Utc>,

    /// When the contract ends (optional)
    pub contract_end: Option<DateTime<Utc>>,

    /// Current status of the relationship
    #[serde(default)]
    pub status: RelationshipStatus,

    /// When the relationship was created
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipType {
    #[serde(rename = "managed")]
    Managed,
    #[serde(rename = "partner")]
    Partner,
    #[serde(rename = "reseller")]
    Reseller,
}

impl Default for RelationshipType {
    fn default() -> Self {
        RelationshipType::Managed
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BillingArrangement {
    #[serde(rename = "msp_pays")]
    MspPays,
    #[serde(rename = "customer_pays")]
    CustomerPays,
    #[serde(rename = "split")]
    Split,
}

impl Default for BillingArrangement {
    fn default() -> Self {
        BillingArrangement::MspPays
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RelationshipStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "terminated")]
    Terminated,
}

impl Default for RelationshipStatus {
    fn default() -> Self {
        RelationshipStatus::Active
    }
}

impl MspCustomerRelationship {
    /// Create a new MSP-Customer relationship
    pub fn new(
        msp_tenant: Thing,
        customer_tenant: Thing,
        relationship_type: RelationshipType,
        billing_arrangement: BillingArrangement,
    ) -> Self {
        Self {
            id: None,
            msp_tenant,
            customer_tenant,
            relationship_type,
            billing_arrangement,
            commission_rate: None,
            contract_start: chrono::Utc::now(),
            contract_end: None,
            status: RelationshipStatus::Active,
            created_at: None, // Will be set by SurrealDB
        }
    }

    /// Create a new relationship with commission rate
    pub fn new_with_commission(
        msp_tenant: Thing,
        customer_tenant: Thing,
        relationship_type: RelationshipType,
        billing_arrangement: BillingArrangement,
        commission_rate: f64,
    ) -> Self {
        let mut relationship = Self::new(
            msp_tenant,
            customer_tenant,
            relationship_type,
            billing_arrangement,
        );
        relationship.commission_rate = Some(commission_rate);
        relationship
    }

    /// Create a relationship with contract end date
    pub fn new_with_contract_end(
        msp_tenant: Thing,
        customer_tenant: Thing,
        relationship_type: RelationshipType,
        billing_arrangement: BillingArrangement,
        contract_end: DateTime<Utc>,
    ) -> Self {
        let mut relationship = Self::new(
            msp_tenant,
            customer_tenant,
            relationship_type,
            billing_arrangement,
        );
        relationship.contract_end = Some(contract_end);
        relationship
    }

    /// Check if the relationship is currently active
    pub fn is_active(&self) -> bool {
        matches!(self.status, RelationshipStatus::Active)
    }

    /// Check if the contract has expired
    pub fn is_contract_expired(&self) -> bool {
        if let Some(end_date) = self.contract_end {
            chrono::Utc::now() > end_date
        } else {
            false
        }
    }

    /// Suspend the relationship
    pub fn suspend(&mut self) {
        self.status = RelationshipStatus::Suspended;
    }

    /// Terminate the relationship
    pub fn terminate(&mut self) {
        self.status = RelationshipStatus::Terminated;
    }

    /// Reactivate the relationship
    pub fn reactivate(&mut self) {
        self.status = RelationshipStatus::Active;
    }

    /// Update commission rate
    pub fn set_commission_rate(&mut self, rate: f64) {
        self.commission_rate = Some(rate);
    }

    /// Remove commission rate
    pub fn clear_commission_rate(&mut self) {
        self.commission_rate = None;
    }

    /// Extend contract end date
    pub fn extend_contract(&mut self, new_end_date: DateTime<Utc>) {
        self.contract_end = Some(new_end_date);
    }

    /// Make contract permanent (remove end date)
    pub fn make_contract_permanent(&mut self) {
        self.contract_end = None;
    }

    /// Get contract duration in days
    pub fn contract_duration_days(&self) -> Option<i64> {
        self.contract_end
            .map(|end| (end - self.contract_start).num_days())
    }

    /// Get time remaining on contract
    pub fn time_remaining(&self) -> Option<chrono::Duration> {
        self.contract_end.map(|end| end - chrono::Utc::now())
    }

    /// Check if contract expires soon (within threshold)
    pub fn contract_expires_soon(&self, threshold: chrono::Duration) -> bool {
        if let Some(remaining) = self.time_remaining() {
            remaining <= threshold && remaining > chrono::Duration::zero()
        } else {
            false
        }
    }

    /// Get display string for relationship type
    pub fn relationship_type_display(&self) -> &'static str {
        match self.relationship_type {
            RelationshipType::Managed => "Managed Service",
            RelationshipType::Partner => "Partner",
            RelationshipType::Reseller => "Reseller",
        }
    }

    /// Get display string for billing arrangement
    pub fn billing_arrangement_display(&self) -> &'static str {
        match self.billing_arrangement {
            BillingArrangement::MspPays => "MSP Pays",
            BillingArrangement::CustomerPays => "Customer Pays",
            BillingArrangement::Split => "Split Billing",
        }
    }
}
