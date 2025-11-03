DEFINE FUNCTION OVERWRITE fn::signup_customer_tenant($first_name: string, $last_name: string,$tenant_name: string,$email: string, $password: string, $settings: object ) {

    // Create a slug for the tenant name
    // - This gives us a human URL-friendly string
    let $randomised_tenant_name = string::join("-",string::slug($tenant_name),rand::string(7));
    let $tenant_slug = string::slug($randomised_tenant_name);

    // Create a customer type tenant
    let $tenant = CREATE ONLY tenant CONTENT {
        name: $tenant_name,
        slug: $tenant_slug,
        tenant_type: "customer",
        contact_email: $email,
        billing_enabled: true,
        billing_responsibility: "self",
        status: "active",
        settings: $settings,
        parent_tenant: tenant:super_admin,
        created_at: time::now(),
        updated_at: time::now()
    };

    // Create the user record
    // - Ensure the new tenant is an accessible tenant and primary tenant
    let $user = CREATE ONLY user CONTENT {
        email: $email,
        password_hash: crypto::argon2::generate($password),
        first_name: $first_name,
        last_name: $last_name,
        is_active: $is_active,
        signup_completed: false,
        primary_tenant: $tenant.id,
        accessible_tenants: [$tenant.id],
         password_must_change: false,
         password_changed_at: time::now(),
         created_at: time::now(),
         updated_at: time::now()
    };    

    // Assign the tenant admin role to the user
    // - for this customer tenant
    let $tenant_admin_role = CREATE ONLY user_role CONTENT {
        user: $user.id,
        role: role:tenant_admin,
        tenant: $tenant.id,
        granted_by: user:super_admin,
        granted_at: time::now(),
        expires_at: None,
        is_active: true
    };

    RETURN {
        user: $user,
        tenant: $tenant
    };
};

DEFINE FUNCTION OVERWRITE fn::signup_msp_tenant($first_name: string, $last_name: string,$tenant_name: string,$email: string, $password: string, $settings: object ) {

    // Create a slug for the tenant name
    // - This gives us a human URL-friendly string
    let $randomised_tenant_name = string::join("-",string::slug($tenant_name),rand::string(7));
    let $tenant_slug = string::slug($randomised_tenant_name);

    // Create the customer tenant
    let $tenant = CREATE ONLY tenant CONTENT {
        name: $tenant_name,
        slug: $tenant_slug,
        tenant_type: "msp",
        contact_email: $email,
        billing_enabled: true,
        billing_responsibility: "self",
        status: "active",
        settings: $settings,
        parent_tenant: tenant:super_admin,
        created_at: time::now(),
        updated_at: time::now()
    };

    // Create the user record
    // - Default tenant admin for the customer tenant
    let $user = CREATE ONLY user CONTENT {
        email: $email,
        password_hash: crypto::argon2::generate($password),
        first_name: $first_name,
        last_name: $last_name,
        is_active: $is_active,
        signup_completed: false,
        primary_tenant: $tenant.id,
        accessible_tenants: [$tenant.id],
         password_must_change: false,
         password_changed_at: time::now(),
         created_at: time::now(),
         updated_at: time::now()
    };    

    // Assign the tenant admin role to the user
    // - for this customer tenant
    let $msp_admin_role = CREATE ONLY user_role CONTENT {
        user: $user.id,
        role: role:msp_admin,
        tenant: $tenant.id,
        granted_by: user:super_admin,
        granted_at: time::now(),
        expires_at: None,
        is_active: true
    };

    RETURN {
        user: $user,
        tenant: $tenant
    };
};

DEFINE FUNCTION OVERWRITE fn::add_new_msp_customer_tenant($first_name: string, $last_name: string,$tenant_name: string,$email: string, $password: string, $settings: object, $msp_tenant: record<tenant>, $is_msp_billing: bool) {

    let $assumed_msp_admin_id = $session.rd;
    let $assumed_msp_admin = SELECT * FROM $assumed_msp_admin;
    return $assumed_msp_admin;    
};

