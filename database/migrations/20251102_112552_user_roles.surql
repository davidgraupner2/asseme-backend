-- Assign roles to users
INSERT INTO user_role (user, role, tenant, granted_by) VALUES
(user:super_admin, role:super_admin, tenant:super_admin, user:super_admin),
(user:msp_admin_techcorp, role:msp_admin, tenant:msp_techcorp, user:super_admin),
(user:msp_admin_cloudpro, role:msp_admin, tenant:msp_cloudpro, user:super_admin),
(user:acme_admin, role:tenant_admin, tenant:acme_corp, user:msp_admin_techcorp),
(user:widgets_admin, role:tenant_admin, tenant:widgets_inc, user:msp_admin_techcorp),
(user:enterprise_admin, role:tenant_admin, tenant:enterprise_direct, user:super_admin);
