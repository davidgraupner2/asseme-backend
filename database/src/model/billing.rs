use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// Billing model with MSP considerations
///
/// This table manages billing records for tenants with support for
/// MSP billing arrangements and commission tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Billing {
    /// The unique identifier for the billing record
    pub id: Option<Thing>,

    /// The tenant this billing record is for
    pub tenant: Thing,

    /// Who is responsible for paying this bill
    pub responsible_party: Thing,

    /// Start of the billing period
    pub billing_period_start: DateTime<Utc>,

    /// End of the billing period
    pub billing_period_end: DateTime<Utc>,

    /// Amount to be billed
    pub amount: f64,

    /// Currency code (e.g., "USD", "EUR")
    #[serde(default = "default_currency")]
    pub currency: String,

    /// Current status of the billing record
    #[serde(default)]
    pub status: BillingStatus,

    /// Invoice number for tracking
    pub invoice_number: String,

    /// MSP commission amount (optional)
    pub msp_commission: Option<f64>,

    /// When payment is due
    pub payment_due_date: DateTime<Utc>,

    /// When the billing record was created
    pub created_at: Option<DateTime<Utc>>,

    /// When the billing record was last updated
    pub updated_at: Option<DateTime<Utc>>,
}

fn default_currency() -> String {
    "USD".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BillingStatus {
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "paid")]
    Paid,
    #[serde(rename = "overdue")]
    Overdue,
    #[serde(rename = "cancelled")]
    Cancelled,
}

impl Default for BillingStatus {
    fn default() -> Self {
        BillingStatus::Draft
    }
}

impl Billing {
    /// Create a new billing record
    pub fn new(
        tenant: Thing,
        responsible_party: Thing,
        billing_period_start: DateTime<Utc>,
        billing_period_end: DateTime<Utc>,
        amount: f64,
        invoice_number: String,
        payment_due_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id: None,
            tenant,
            responsible_party,
            billing_period_start,
            billing_period_end,
            amount,
            currency: "USD".to_string(),
            status: BillingStatus::Draft,
            invoice_number,
            msp_commission: None,
            payment_due_date,
            created_at: None, // Will be set by SurrealDB
            updated_at: None, // Will be set by SurrealDB
        }
    }

    /// Create a billing record with MSP commission
    pub fn new_with_commission(
        tenant: Thing,
        responsible_party: Thing,
        billing_period_start: DateTime<Utc>,
        billing_period_end: DateTime<Utc>,
        amount: f64,
        invoice_number: String,
        payment_due_date: DateTime<Utc>,
        msp_commission: f64,
    ) -> Self {
        let mut billing = Self::new(
            tenant,
            responsible_party,
            billing_period_start,
            billing_period_end,
            amount,
            invoice_number,
            payment_due_date,
        );
        billing.msp_commission = Some(msp_commission);
        billing
    }

    /// Check if the billing record is paid
    pub fn is_paid(&self) -> bool {
        matches!(self.status, BillingStatus::Paid)
    }

    /// Check if the billing record is overdue
    pub fn is_overdue(&self) -> bool {
        matches!(self.status, BillingStatus::Overdue)
            || (matches!(self.status, BillingStatus::Pending)
                && chrono::Utc::now() > self.payment_due_date)
    }

    /// Check if the billing record is cancelled
    pub fn is_cancelled(&self) -> bool {
        matches!(self.status, BillingStatus::Cancelled)
    }

    /// Mark the billing record as paid
    pub fn mark_paid(&mut self) {
        self.status = BillingStatus::Paid;
    }

    /// Mark the billing record as pending
    pub fn mark_pending(&mut self) {
        self.status = BillingStatus::Pending;
    }

    /// Mark the billing record as overdue
    pub fn mark_overdue(&mut self) {
        self.status = BillingStatus::Overdue;
    }

    /// Cancel the billing record
    pub fn cancel(&mut self) {
        self.status = BillingStatus::Cancelled;
    }

    /// Calculate net amount after MSP commission
    pub fn net_amount(&self) -> f64 {
        if let Some(commission) = self.msp_commission {
            self.amount - commission
        } else {
            self.amount
        }
    }

    /// Calculate commission percentage
    pub fn commission_percentage(&self) -> Option<f64> {
        self.msp_commission
            .map(|commission| (commission / self.amount) * 100.0)
    }

    /// Set MSP commission
    pub fn set_msp_commission(&mut self, commission: f64) {
        self.msp_commission = Some(commission);
    }

    /// Set MSP commission by percentage
    pub fn set_msp_commission_percentage(&mut self, percentage: f64) {
        let commission = (self.amount * percentage) / 100.0;
        self.msp_commission = Some(commission);
    }

    /// Remove MSP commission
    pub fn clear_msp_commission(&mut self) {
        self.msp_commission = None;
    }

    /// Get days until due
    pub fn days_until_due(&self) -> i64 {
        (self.payment_due_date - chrono::Utc::now()).num_days()
    }

    /// Get days overdue (negative if not overdue)
    pub fn days_overdue(&self) -> i64 {
        (chrono::Utc::now() - self.payment_due_date).num_days()
    }

    /// Get billing period duration in days
    pub fn billing_period_days(&self) -> i64 {
        (self.billing_period_end - self.billing_period_start).num_days()
    }

    /// Check if payment is due soon (within threshold)
    pub fn due_soon(&self, threshold_days: i64) -> bool {
        let days_until = self.days_until_due();
        days_until <= threshold_days && days_until > 0
    }

    /// Get formatted amount with currency
    pub fn formatted_amount(&self) -> String {
        format!("{:.2} {}", self.amount, self.currency)
    }

    /// Get formatted commission with currency
    pub fn formatted_commission(&self) -> Option<String> {
        self.msp_commission
            .map(|commission| format!("{:.2} {}", commission, self.currency))
    }

    /// Get formatted net amount with currency
    pub fn formatted_net_amount(&self) -> String {
        format!("{:.2} {}", self.net_amount(), self.currency)
    }

    /// Update currency
    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }

    /// Extend payment due date
    pub fn extend_due_date(&mut self, new_due_date: DateTime<Utc>) {
        self.payment_due_date = new_due_date;
    }

    /// Get status display string
    pub fn status_display(&self) -> &'static str {
        match self.status {
            BillingStatus::Draft => "Draft",
            BillingStatus::Pending => "Pending",
            BillingStatus::Paid => "Paid",
            BillingStatus::Overdue => "Overdue",
            BillingStatus::Cancelled => "Cancelled",
        }
    }
}
