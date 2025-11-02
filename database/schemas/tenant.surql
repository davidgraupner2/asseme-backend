DEFINE TABLE OVERWRITE tenant SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE fn::has_permission(<string>$auth.id, 'tenant.view_all', NONE) OR 
                        fn::has_permission(<string>$auth.id, 'tenant.view_own', <string>id) OR
                        fn::has_permission(<string>$auth.id, 'tenant.view_customers', <string>id)
        FOR create WHERE fn::has_permission(<string>$auth.id, 'tenant.create', NONE)
        FOR update WHERE fn::has_permission(<string>$auth.id, 'tenant.manage_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'tenant.update_own', <string>id) OR
                        fn::has_permission(<string>$auth.id, 'tenant.update_customers', <string>id)
        FOR delete WHERE fn::has_permission(<string>$auth.id, 'tenant.delete', NONE);
DEFINE FIELD OVERWRITE name ON TABLE tenant TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE slug ON TABLE tenant TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE tenant_type ON TABLE tenant TYPE string ASSERT $value IN ['super_admin', 'msp', 'customer'] DEFAULT 'customer';
DEFINE FIELD OVERWRITE parent_tenant ON TABLE tenant TYPE option<record<tenant>>;
DEFINE FIELD OVERWRITE msp_tenant ON TABLE tenant TYPE option<record<tenant>>;
DEFINE FIELD OVERWRITE contact_email ON TABLE tenant TYPE string;
DEFINE FIELD OVERWRITE contact_phone ON TABLE tenant TYPE option<string>;
DEFINE FIELD OVERWRITE billing_enabled ON TABLE tenant TYPE bool DEFAULT true;
DEFINE FIELD OVERWRITE billing_responsibility ON TABLE tenant TYPE string ASSERT $value IN ['self', 'msp', 'parent'] DEFAULT 'self';
DEFINE FIELD OVERWRITE status ON TABLE tenant TYPE string ASSERT $value IN ['active', 'suspended', 'inactive'] DEFAULT 'active';
DEFINE FIELD OVERWRITE settings ON TABLE tenant TYPE object DEFAULT {};
DEFINE FIELD OVERWRITE created_at ON TABLE tenant TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON TABLE tenant TYPE datetime DEFAULT time::now();