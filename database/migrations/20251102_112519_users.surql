-- Create users for different levels
-- Default Super Admin with temporary password that must be changed
INSERT INTO user (id, email, password_hash, password_must_change, first_name, last_name, phone, primary_tenant, accessible_tenants, password_changed_at) VALUES
('super_admin', 'super@admin.com', crypto::argon2::generate('TempPassword123!'), true, 'Super', 'Admin', '+1-800-ADMIN', tenant:super_admin, [tenant:super_admin], NONE);

-- Sample MSP and Customer users (for demonstration - these would normally be created via signup)
INSERT INTO user (id, email, password_hash, password_must_change, first_name, last_name, phone, primary_tenant, accessible_tenants, password_changed_at) VALUES
('msp_admin_techcorp', 'admin@techcorp.com', crypto::argon2::generate('SamplePassword123!'), false, 'MSP', 'Admin', '+1-555-MSP1', tenant:msp_techcorp, [tenant:msp_techcorp, tenant:acme_corp, tenant:widgets_inc], time::now()),
('msp_admin_cloudpro', 'admin@cloudpro.com', crypto::argon2::generate('SamplePassword123!'), false, 'Cloud', 'Admin', '+1-555-MSP2', tenant:msp_cloudpro, [tenant:msp_cloudpro, tenant:startup_xyz], time::now()),
('acme_admin', 'admin@acme.com', crypto::argon2::generate('SamplePassword123!'), false, 'Acme', 'Admin', '+1-555-ACME1', tenant:acme_corp, [tenant:acme_corp], time::now()),
('widgets_admin', 'admin@widgets.com', crypto::argon2::generate('SamplePassword123!'), false, 'Widgets', 'Admin', '+1-555-WIDGET1', tenant:widgets_inc, [tenant:widgets_inc], time::now()),
('enterprise_admin', 'admin@enterprise.com', crypto::argon2::generate('SamplePassword123!'), false, 'Enterprise', 'Admin', '+1-555-ENTER1', tenant:enterprise_direct, [tenant:enterprise_direct], time::now());
