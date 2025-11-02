DEFINE TABLE OVERWRITE user SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE $auth.id = id OR 
                        fn::has_permission(<string>$auth.id, 'user.view_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'user.view_own_tenant', <string>primary_tenant) OR
                        fn::has_permission(<string>$auth.id, 'user.view_customers', <string>primary_tenant)
        FOR create WHERE fn::has_permission(<string>$auth.id, 'user.create_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'user.create_own_tenant', <string>primary_tenant) OR
                        fn::has_permission(<string>$auth.id, 'user.create_customers', <string>primary_tenant)
        FOR update WHERE $auth.id = id OR
                        fn::has_permission(<string>$auth.id, 'user.update_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'user.update_own_tenant', <string>primary_tenant) OR
                        fn::has_permission(<string>$auth.id, 'user.update_customers', <string>primary_tenant)
        FOR delete WHERE fn::has_permission(<string>$auth.id, 'user.delete_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'user.delete_own_tenant', <string>primary_tenant) OR
                        fn::has_permission(<string>$auth.id, 'user.delete_customers', <string>primary_tenant);
DEFINE FIELD OVERWRITE email ON TABLE user TYPE string ASSERT string::is::email($value);
DEFINE FIELD OVERWRITE password_hash ON TABLE user TYPE string ASSERT $value != NONE
    PERMISSIONS FOR select WHERE $auth.id = id;
DEFINE FIELD OVERWRITE password_must_change ON TABLE user TYPE bool DEFAULT false;
DEFINE FIELD OVERWRITE first_name ON TABLE user TYPE string;
DEFINE FIELD OVERWRITE last_name ON TABLE user TYPE string;
DEFINE FIELD OVERWRITE phone ON TABLE user TYPE option<string>;
DEFINE FIELD OVERWRITE primary_tenant ON TABLE user TYPE record<tenant>;
DEFINE FIELD OVERWRITE accessible_tenants ON TABLE user TYPE array<record<tenant>> DEFAULT [];
DEFINE FIELD OVERWRITE is_active ON TABLE user TYPE bool DEFAULT true;
DEFINE FIELD OVERWRITE last_login ON TABLE user TYPE option<datetime>;
DEFINE FIELD OVERWRITE password_changed_at ON TABLE user TYPE option<datetime>;
DEFINE FIELD OVERWRITE created_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON TABLE user TYPE datetime VALUE time::now();