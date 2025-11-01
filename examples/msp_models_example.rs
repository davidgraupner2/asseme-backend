/// Example usage of the MSP multi-tenant database models
/// 
/// This example demonstrates how to use all the models created from the
/// SurrealDB schema, including tenants, users, roles, billing, and relationships.

use database::{
    context::{get_initialized_database, check_database_health},
    model::{
        tenant::{Tenant, TenantType},
        user::User,
        role::{Role, RoleType},
        user_role::UserRole,
        msp_customer_relationship::{MspCustomerRelationship, RelationshipType, BillingArrangement},
        billing::{Billing, BillingStatus},
        token::{Token, TokenType},
    }
};
use surrealdb::sql::Thing;
use chrono::{DateTime, Utc, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::init();
    
    println!("🚀 MSP Database Models Example");
    println!("==============================");
    
    // 1. Initialize database with schema
    println!("\n📊 Connecting to database and initializing schema...");
    let db = get_initialized_database(
        "ws".to_string(),
        "localhost:8000".to_string(),
        "root".to_string(),
        "root".to_string(),
        "asseme".to_string(),
        "msp".to_string(),
        false, // Don't force reinit
    ).await?;
    
    // 2. Check database health
    println!("\n🏥 Checking database health...");
    let health = check_database_health(&db).await?;
    println!("   {}", health.summary());
    
    if !health.is_healthy() {
        println!("   ❌ Database is not healthy!");
        for error in &health.errors {
            println!("   Error: {}", error);
        }
        for warning in &health.warnings {
            println!("   Warning: {}", warning);
        }
        return Ok(());
    }
    
    println!("   ✅ Database is healthy!");
    for (table, count) in &health.table_counts {
        println!("   📋 {}: {} records", table, count);
    }
    
    // 3. Demonstrate model usage
    println!("\n🏢 Creating example tenant...");
    let tenant = Tenant::new(
        "Example Corp".to_string(),
        "example-corp".to_string(),
        "admin@example.com".to_string(),
    );
    println!("   Created tenant: {} ({})", tenant.name, tenant.slug);
    println!("   Type: {:?}, Status: {:?}", tenant.tenant_type, tenant.status);
    
    // 4. Create MSP tenant
    println!("\n🔧 Creating MSP tenant...");
    let msp_tenant = Tenant::new_msp(
        "TechCorp MSP".to_string(),
        "techcorp-msp".to_string(),
        "msp@techcorp.com".to_string(),
    );
    println!("   Created MSP: {} ({})", msp_tenant.name, msp_tenant.slug);
    println!("   Is MSP: {}", msp_tenant.is_msp());
    
    // 5. Create customer under MSP
    println!("\n👥 Creating customer tenant...");
    let customer_tenant = Tenant::new_customer(
        "Customer Inc".to_string(),
        "customer-inc".to_string(),
        "admin@customer.com".to_string(),
        Thing::from(("tenant", "msp_id")),
    );
    println!("   Created customer: {} ({})", customer_tenant.name, customer_tenant.slug);
    println!("   Is customer: {}", customer_tenant.is_customer());
    
    // 6. Create users
    println!("\n👤 Creating example users...");
    let admin_user = User::new(
        "admin@example.com".to_string(),
        "John".to_string(),
        "Admin".to_string(),
        "hashed_password_123".to_string(),
        Thing::from(("tenant", "example_tenant")),
    );
    println!("   Created user: {} ({})", admin_user.full_name(), admin_user.email);
    println!("   Is active: {}", admin_user.is_active());
    
    let temp_user = User::new_with_temp_password(
        "temp@example.com".to_string(),
        "Jane".to_string(),
        "Temp".to_string(),
        "temp_password_hash".to_string(),
        Thing::from(("tenant", "example_tenant")),
    );
    println!("   Created temp user: {} (must change password: {})", 
             temp_user.full_name(), temp_user.must_change_password());
    
    // 7. Create roles
    println!("\n🔐 Creating example roles...");
    let admin_role = Role::new(
        "Example Admin".to_string(),
        "Administrator for example tenant".to_string(),
        3,
        RoleType::TenantAdmin.default_permissions(),
    );
    println!("   Created role: {} (level {})", admin_role.name, admin_role.level);
    println!("   Permissions: {}", admin_role.permissions.len());
    
    // 8. Create user-role assignment
    println!("\n🔗 Creating user-role assignment...");
    let user_role = UserRole::new(
        Thing::from(("user", "admin_user")),
        Thing::from(("role", "admin_role")),
        Thing::from(("tenant", "example_tenant")),
        Thing::from(("user", "super_admin")),
    );
    println!("   Assigned role at: {}", user_role.granted_at);
    println!("   Is valid: {}", user_role.is_valid());
    
    // 9. Create MSP-Customer relationship
    println!("\n🤝 Creating MSP-Customer relationship...");
    let relationship = MspCustomerRelationship::new_with_commission(
        Thing::from(("tenant", "msp_tenant")),
        Thing::from(("tenant", "customer_tenant")),
        RelationshipType::Managed,
        BillingArrangement::MspPays,
        15.0, // 15% commission
    );
    println!("   Created relationship: {} with {}% commission", 
             relationship.relationship_type_display(), 
             relationship.commission_rate.unwrap_or(0.0));
    println!("   Billing: {}", relationship.billing_arrangement_display());
    
    // 10. Create billing record
    println!("\n💳 Creating billing record...");
    let billing = Billing::new_with_commission(
        Thing::from(("tenant", "customer_tenant")),
        Thing::from(("tenant", "msp_tenant")),
        Utc::now() - Duration::days(30),
        Utc::now(),
        1000.0,
        "INV-2024-001".to_string(),
        Utc::now() + Duration::days(30),
        150.0, // Commission amount
    );
    println!("   Created invoice: {} for {}", 
             billing.invoice_number, billing.formatted_amount());
    println!("   Commission: {}", billing.formatted_commission().unwrap_or_default());
    println!("   Net amount: {}", billing.formatted_net_amount());
    println!("   Due in {} days", billing.days_until_due());
    
    // 11. Create token
    println!("\n🔑 Creating access token...");
    let token = Token::new_agent_token(
        Thing::from(("tenant", "example_tenant")),
        "jwt_token_string_here".to_string(),
        Thing::from(("user", "admin_user")),
    );
    println!("   Created token for agent access");
    println!("   Types: {}", token.token_types_display());
    println!("   Is valid: {}", token.is_valid());
    
    // 12. Demonstrate model methods
    println!("\n🧪 Demonstrating model methods...");
    
    // User methods
    let mut demo_user = admin_user.clone();
    demo_user.add_tenant_access(Thing::from(("tenant", "other_tenant")));
    println!("   User accessible tenants: {}", demo_user.all_accessible_tenants().len());
    
    // Tenant methods
    let mut demo_tenant = tenant.clone();
    demo_tenant.set_setting("theme".to_string(), serde_json::Value::String("dark".to_string()));
    println!("   Tenant settings: {}", demo_tenant.settings.len());
    
    // Role methods
    let mut demo_role = admin_role.clone();
    demo_role.add_permission("custom.permission".to_string());
    println!("   Role permissions: {}", demo_role.permissions.len());
    
    // Billing methods
    let mut demo_billing = billing.clone();
    demo_billing.set_msp_commission_percentage(20.0);
    println!("   Updated commission to {}%", demo_billing.commission_percentage().unwrap_or(0.0));
    
    println!("\n✅ All models working correctly!");
    println!("🎉 MSP Database Models Example Complete!");
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_model_creation() {
        // Test basic model creation without database
        let tenant = Tenant::new(
            "Test Tenant".to_string(),
            "test-tenant".to_string(),
            "test@example.com".to_string(),
        );
        assert_eq!(tenant.name, "Test Tenant");
        assert_eq!(tenant.slug, "test-tenant");
        assert!(tenant.is_customer()); // Default type
        
        let user = User::new(
            "user@test.com".to_string(),
            "Test".to_string(),
            "User".to_string(),
            "password_hash".to_string(),
            Thing::from(("tenant", "test")),
        );
        assert_eq!(user.email, "user@test.com");
        assert_eq!(user.full_name(), "Test User");
        assert!(user.is_active());
    }
    
    #[test] 
    fn test_role_permissions() {
        let role = Role::new(
            "Test Role".to_string(),
            "Test role description".to_string(),
            3,
            vec!["test.permission".to_string()],
        );
        
        assert!(role.has_permission("test.permission"));
        assert!(!role.has_permission("other.permission"));
    }
    
    #[test]
    fn test_billing_calculations() {
        let billing = Billing::new_with_commission(
            Thing::from(("tenant", "customer")),
            Thing::from(("tenant", "msp")),
            Utc::now(),
            Utc::now(),
            1000.0,
            "TEST-001".to_string(),
            Utc::now(),
            100.0, // 10% commission
        );
        
        assert_eq!(billing.net_amount(), 900.0);
        assert_eq!(billing.commission_percentage(), Some(10.0));
    }
}