DEFINE TABLE OVERWRITE token TYPE NORMAL SCHEMAFULL 
    PERMISSIONS 
        FOR select WHERE fn::has_permission(<string>$auth.id, 'token.view_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'token.view_own', <string>tenant) OR
                        fn::has_permission(<string>$auth.id, 'token.view_customers', <string>tenant)
        FOR create WHERE fn::has_permission(<string>$auth.id, 'token.create_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'token.create_customers', <string>tenant)
        FOR update WHERE fn::has_permission(<string>$auth.id, 'token.update_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'token.update_own', <string>tenant) OR
                        fn::has_permission(<string>$auth.id, 'token.update_customers', <string>tenant)
        FOR delete WHERE fn::has_permission(<string>$auth.id, 'token.delete_all', NONE);

DEFINE FIELD OVERWRITE tenant ON token TYPE record<tenant>;
DEFINE FIELD OVERWRITE type ON token TYPE set<'agent'>;
DEFINE FIELD OVERWRITE jwt ON token TYPE string;
DEFINE FIELD OVERWRITE type[*] ON token TYPE 'agent' PERMISSIONS FULL;
DEFINE FIELD OVERWRITE owner ON token TYPE record<user>;
DEFINE FIELD OVERWRITE created_at ON TABLE billing TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON TABLE billing TYPE datetime VALUE time::now();

-- Indexes for performance
DEFINE INDEX OVERWRITE tenant_slug_idx ON TABLE tenant COLUMNS slug UNIQUE;
DEFINE INDEX OVERWRITE user_email_idx ON TABLE user COLUMNS email UNIQUE;
DEFINE INDEX OVERWRITE user_tenant_idx ON TABLE user_role COLUMNS user, tenant;
DEFINE INDEX OVERWRITE msp_customer_idx ON TABLE msp_customer_relationship COLUMNS msp_tenant, customer_tenant UNIQUE;
