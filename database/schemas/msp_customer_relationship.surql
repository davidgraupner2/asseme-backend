DEFINE TABLE OVERWRITE msp_customer_relationship SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE fn::has_permission(<string>$auth.id, 'msp.view_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'msp.view_own', <string>msp_tenant)
        FOR create, update, delete WHERE fn::has_permission(<string>$auth.id, 'msp.manage_relationships', <string>msp_tenant) OR
                                        fn::has_permission(<string>$auth.id, 'system.manage', NONE);
DEFINE FIELD OVERWRITE msp_tenant ON TABLE msp_customer_relationship TYPE record<tenant>;
DEFINE FIELD OVERWRITE customer_tenant ON TABLE msp_customer_relationship TYPE record<tenant>;
DEFINE FIELD OVERWRITE relationship_type ON TABLE msp_customer_relationship TYPE string ASSERT $value IN ['managed', 'partner', 'reseller'] DEFAULT 'managed';
DEFINE FIELD OVERWRITE billing_arrangement ON TABLE msp_customer_relationship TYPE string ASSERT $value IN ['msp_pays', 'customer_pays', 'split'] DEFAULT 'msp_pays';
DEFINE FIELD OVERWRITE commission_rate ON TABLE msp_customer_relationship TYPE option<number>;
DEFINE FIELD OVERWRITE contract_start ON TABLE msp_customer_relationship TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE contract_end ON TABLE msp_customer_relationship TYPE option<datetime>;
DEFINE FIELD OVERWRITE status ON TABLE msp_customer_relationship TYPE string ASSERT $value IN ['active', 'suspended', 'terminated'] DEFAULT 'active';
DEFINE FIELD OVERWRITE created_at ON TABLE msp_customer_relationship TYPE datetime DEFAULT time::now();