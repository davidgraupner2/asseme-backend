-- Create the super admin tenant (root level)
INSERT INTO tenant (id, name, slug, tenant_type, parent_tenant, msp_tenant, contact_email, contact_phone, billing_responsibility) VALUES
('tenant:super_admin', 'System Administration', 'super-admin', 'super_admin', NONE, NONE, 'admin@system.com', '+1-800-SYSTEM', 'self');

-- Create sample MSP tenants
INSERT INTO tenant (id, name, slug, tenant_type, parent_tenant, msp_tenant, contact_email, contact_phone, billing_responsibility) VALUES
('tenant:msp_techcorp', 'TechCorp MSP', 'techcorp-msp', 'msp', tenant:super_admin, NONE, 'admin@techcorp.com', '+1-555-TECH', 'self'),
('tenant:msp_cloudpro', 'CloudPro Services', 'cloudpro-msp', 'msp', tenant:super_admin, NONE, 'billing@cloudpro.com', '+1-555-CLOUD', 'self');

-- Create sample customer tenants under MSPs
INSERT INTO tenant (id, name, slug, tenant_type, parent_tenant, msp_tenant, contact_email, contact_phone, billing_responsibility) VALUES
('tenant:acme_corp', 'Acme Corporation', 'acme-corp', 'customer', tenant:msp_techcorp, tenant:msp_techcorp, 'admin@acme.com', '+1-555-ACME', 'msp'),
('tenant:widgets_inc', 'Widgets Inc', 'widgets-inc', 'customer', tenant:msp_techcorp, tenant:msp_techcorp, 'it@widgets.com', '+1-555-WIDGET', 'self'),
('tenant:startup_xyz', 'Startup XYZ', 'startup-xyz', 'customer', tenant:msp_cloudpro, tenant:msp_cloudpro, 'tech@startupxyz.com', '+1-555-START', 'msp');

-- Create independent customer (direct customer, not through MSP)
INSERT INTO tenant (id, name, slug, tenant_type, parent_tenant, msp_tenant, contact_email, contact_phone, billing_responsibility) VALUES
('tenant:enterprise_direct', 'Enterprise Direct', 'enterprise-direct', 'customer', tenant:super_admin, NONE, 'admin@enterprise.com', '+1-555-ENTER', 'self');