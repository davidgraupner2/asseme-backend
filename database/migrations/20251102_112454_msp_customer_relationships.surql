-- Create MSP-Customer relationships
INSERT INTO msp_customer_relationship (msp_tenant, customer_tenant, relationship_type, billing_arrangement, commission_rate) VALUES
(tenant:msp_techcorp, tenant:acme_corp, 'managed', 'msp_pays', 0.15),
(tenant:msp_techcorp, tenant:widgets_inc, 'managed', 'customer_pays', 0.10),
(tenant:msp_cloudpro, tenant:startup_xyz, 'managed', 'msp_pays', 0.20);
