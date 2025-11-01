# MSP Database Models - Usage Guide

This document explains how to use the complete MSP multi-tenant database system that was generated from your SurrealDB schema.

## 🚀 Quick Start

### 1. Database Initialization

The system automatically initializes your complete schema when you connect:

```rust
use database::context::get_initialized_database;

// Connect and auto-initialize schema
let db = get_initialized_database(
    "ws".to_string(),
    "localhost:8000".to_string(),
    "root".to_string(),
    "root".to_string(),
    "asseme".to_string(),
    "msp".to_string(),
    false, // Don't force reinit if schema exists
).await?;
```

### 2. Health Check

```rust
use database::context::check_database_health;

let health = check_database_health(&db).await?;
println!("{}", health.summary());

// Check table counts
for (table, count) in &health.table_counts {
    println!("{}: {} records", table, count);
}
```

## 📊 Available Models

All models correspond to your SurrealDB tables and include helper methods:

### Tenant Management

```rust
use database::model::tenant::{Tenant, TenantType};

// Create basic tenant
let tenant = Tenant::new(
    "Acme Corp".to_string(),
    "acme-corp".to_string(),
    "admin@acme.com".to_string()
);

// Create MSP tenant
let msp = Tenant::new_msp(
    "TechCorp MSP".to_string(),
    "techcorp-msp".to_string(),
    "contact@techcorp.com".to_string()
);

// Create customer under MSP
let customer = Tenant::new_customer(
    "Customer Inc".to_string(),
    "customer-inc".to_string(),
    "admin@customer.com".to_string(),
    Thing::from(("tenant", "msp_id"))
);

// Helper methods
println!("Is MSP: {}", msp.is_msp());
println!("Is active: {}", tenant.is_active());
tenant.set_setting("theme".to_string(), json!("dark"));
```

### User Management

```rust
use database::model::user::User;

// Create user
let user = User::new(
    "john@example.com".to_string(),
    "John".to_string(),
    "Doe".to_string(),
    "hashed_password".to_string(),
    Thing::from(("tenant", "company_id"))
);

// Create user requiring password change
let temp_user = User::new_with_temp_password(
    "temp@example.com".to_string(),
    "Jane".to_string(),
    "Temp".to_string(),
    "temp_hash".to_string(),
    Thing::from(("tenant", "company_id"))
);

// Helper methods
println!("Full name: {}", user.full_name());
println!("Must change password: {}", temp_user.must_change_password());
user.add_tenant_access(Thing::from(("tenant", "other_tenant")));
user.record_login();
user.update_password("new_hashed_password".to_string());
```

### Role Management

```rust
use database::model::role::{Role, RoleType};

// Create custom role
let role = Role::new(
    "Custom Admin".to_string(),
    "Custom administrator role".to_string(),
    3,
    vec!["custom.permission".to_string()]
);

// Use predefined role types
let permissions = RoleType::TenantAdmin.default_permissions();
let level = RoleType::MspAdmin.level();

// Helper methods
println!("Has permission: {}", role.has_permission("custom.permission"));
role.add_permission("new.permission".to_string());
```

### User-Role Assignments

```rust
use database::model::user_role::UserRole;

// Assign role to user in tenant context
let assignment = UserRole::new(
    Thing::from(("user", "user_id")),
    Thing::from(("role", "role_id")),
    Thing::from(("tenant", "tenant_id")),
    Thing::from(("user", "granter_id"))
);

// Temporary assignment
let temp_assignment = UserRole::new_temporary(
    Thing::from(("user", "user_id")),
    Thing::from(("role", "role_id")),
    Thing::from(("tenant", "tenant_id")),
    Thing::from(("user", "granter_id")),
    Utc::now() + Duration::days(30)
);

// Helper methods
println!("Is valid: {}", assignment.is_valid());
println!("Expires soon: {}", temp_assignment.expires_soon(Duration::days(7)));
```

### MSP-Customer Relationships

```rust
use database::model::msp_customer_relationship::{
    MspCustomerRelationship, RelationshipType, BillingArrangement
};

// Create relationship with commission
let relationship = MspCustomerRelationship::new_with_commission(
    Thing::from(("tenant", "msp_id")),
    Thing::from(("tenant", "customer_id")),
    RelationshipType::Managed,
    BillingArrangement::MspPays,
    15.0 // 15% commission
);

// Helper methods
println!("Type: {}", relationship.relationship_type_display());
println!("Billing: {}", relationship.billing_arrangement_display());
println!("Contract expires soon: {}", relationship.contract_expires_soon(Duration::days(30)));
relationship.extend_contract(Utc::now() + Duration::days(365));
```

### Billing Management

```rust
use database::model::billing::{Billing, BillingStatus};

// Create billing record with commission
let billing = Billing::new_with_commission(
    Thing::from(("tenant", "customer_id")),
    Thing::from(("tenant", "msp_id")),
    Utc::now() - Duration::days(30),
    Utc::now(),
    1000.0,
    "INV-2024-001".to_string(),
    Utc::now() + Duration::days(30),
    150.0 // Commission amount
);

// Helper methods
println!("Formatted amount: {}", billing.formatted_amount());
println!("Commission: {}", billing.formatted_commission().unwrap());
println!("Net amount: {}", billing.formatted_net_amount());
println!("Days until due: {}", billing.days_until_due());
println!("Is overdue: {}", billing.is_overdue());

billing.mark_paid();
billing.set_msp_commission_percentage(20.0);
```

### Token Management

```rust
use database::model::token::{Token, TokenType};

// Create agent token
let token = Token::new_agent_token(
    Thing::from(("tenant", "tenant_id")),
    "jwt_token_string".to_string(),
    Thing::from(("user", "owner_id"))
);

// Helper methods
println!("Is agent token: {}", token.is_agent_token());
println!("Token types: {}", token.token_types_display());
println!("Is valid: {}", token.is_valid());
token.update_jwt("new_jwt_string".to_string());
```

## 🔧 Advanced Usage

### Manual Schema Operations

```rust
use database::context::{initialize_schema, reset_database};

// Force schema reinitialization
initialize_schema(&db).await?;

// Reset database (WARNING: Deletes all data!)
let fresh_db = reset_database(
    "ws".to_string(),
    "localhost:8000".to_string(),
    "root".to_string(),
    "root".to_string(),
    "asseme".to_string(),
    "msp".to_string()
).await?;
```

### Database Connection Only

```rust
use database::context::get_database;

// Connect without automatic schema initialization
let db = get_database(
    "ws".to_string(),
    "localhost:8000".to_string(),
    "root".to_string(),
    "root".to_string(),
    "asseme".to_string(),
    "msp".to_string()
).await?;
```

## 📋 Schema Overview

Your complete schema includes:

### Tables Created

- **tenant**: Multi-level tenant hierarchy
- **user**: User management with multi-tenant access
- **role**: 5-level role hierarchy with permissions
- **user_role**: User-role assignments with tenant context
- **msp_customer_relationship**: MSP-customer contracts
- **billing**: Billing records with MSP commission tracking
- **token**: API access and agent authentication

### Sample Data Created

- Super admin user: `super@admin.com` (password must be changed)
- Sample MSP tenants
- Sample customer tenants
- Complete role hierarchy
- Sample user assignments
- MSP-customer relationships

### Functions Available

All SurrealDB utility functions from your schema are created, including:

- Permission checking functions
- User management functions
- Tenant management functions
- Role assignment functions

## 🎯 Integration Example

Here's a complete example showing how to integrate this into your application:

```rust
use database::{
    context::get_initialized_database,
    model::{
        tenant::Tenant,
        user::User,
        role::RoleType,
    }
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database
    let db = get_initialized_database(
        "ws".to_string(),
        "localhost:8000".to_string(),
        "root".to_string(),
        "root".to_string(),
        "asseme".to_string(),
        "msp".to_string(),
        false,
    ).await?;

    // Your application logic here using the models...

    Ok(())
}
```

## ✅ Benefits

✅ **Type Safety**: All models are strongly typed with Rust structs  
✅ **Helper Methods**: Rich API for common operations  
✅ **Schema Management**: Automatic initialization and verification  
✅ **Multi-Tenant**: Complete MSP hierarchy support  
✅ **Permission System**: 5-level role-based access control  
✅ **Billing Support**: MSP commission and billing management  
✅ **Comprehensive**: All your SurrealDB tables and relationships

The database models are now ready to use in your MSP application! 🚀
