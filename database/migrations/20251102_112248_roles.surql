-- Create hierarchical roles with detailed permissions
INSERT INTO role (id, name, description, level, permissions) VALUES
('role:super_admin', 'Super Admin', 'Full system access across all tenants', 1, [
    'system.manage', 'tenant.create', 'tenant.delete', 'tenant.manage_all', 'tenant.view_all',
    'user.create_all', 'user.update_all', 'user.delete_all', 'user.view_all',
    'msp.create', 'msp.update_all', 'msp.delete_all', 'msp.view_all',
    'billing.create_all', 'billing.update_all', 'billing.delete_all', 'billing.view_all',
    'role.assign_all', 'role.view_all',
    'super_admin.create', 'msp_admin.create', 'tenant_admin.create','token.view_all','token.view_own','token.view_customers',
    'token.create_all','token.create_customers','token.update_all','token.update_own','token.update_customers','token.delete_all'
]),
('role:msp_admin', 'MSP Admin', 'Manage MSP tenant and customer tenants', 2, [
    'tenant.view_own', 'tenant.update_own', 'tenant.view_customers', 'tenant.update_customers',
    'user.create_customers', 'user.update_customers', 'user.view_customers', 'user.delete_customers',
    'user.view_own_tenant', 'user.update_own_tenant', 'user.create_own_tenant',
    'msp.view_own', 'msp.update_own', 'msp.manage_relationships',
    'billing.view_customers', 'billing.update_customers', 'billing.create_customers',
    'billing.view_own', 'billing.update_own',
    'role.assign_customers', 'role.assign_own_tenant',
    'msp_admin.create_for_owned_tenants', 'tenant_admin.create_customers', 'user_manager.create_customers'
]),
('role:tenant_admin', 'Tenant Admin', 'Full access within own tenant', 3, [
    'tenant.view_own', 'tenant.update_own', 'tenant.manage_settings',
    'user.create_own_tenant', 'user.update_own_tenant', 'user.view_own_tenant', 'user.delete_own_tenant',
    'billing.view_own', 'billing.update_own',
    'role.assign_own_tenant', 'role.view_own_tenant',
    'tenant_admin.create_own_tenant', 'user_manager.create_own_tenant', 'user.create_own_tenant'
]),
('role:user_manager', 'User Manager', 'Manage users within tenant', 4, [
    'user.create_own_tenant', 'user.update_own_tenant', 'user.view_own_tenant', 'user.delete_basic_own_tenant',
    'role.assign_basic_own_tenant', 'role.view_own_tenant',
    'user.create_own_tenant'
]),
('role:user', 'Standard User', 'Basic user access', 5, [
    'profile.edit', 'profile.view', 'data.view_own', 'settings.view'
]);
