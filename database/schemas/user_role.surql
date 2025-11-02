DEFINE TABLE OVERWRITE user_role SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE $auth.id = user OR
                        fn::has_permission(<string>$auth.id, 'role.view_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'role.view_own_tenant', <string>tenant)
        FOR create, update WHERE fn::has_permission(<string>$auth.id, 'role.assign_all', NONE) OR
                               fn::has_permission(<string>$auth.id, 'role.assign_own_tenant', <string>tenant) OR
                               fn::has_permission(<string>$auth.id, 'role.assign_customers', <string>tenant)
        FOR delete WHERE fn::has_permission(<string>$auth.id, 'role.assign_all', NONE);
DEFINE FIELD OVERWRITE user ON TABLE user_role TYPE record<user>;
DEFINE FIELD OVERWRITE role ON TABLE user_role TYPE record<role>;
DEFINE FIELD OVERWRITE tenant ON TABLE user_role TYPE record<tenant>;
DEFINE FIELD OVERWRITE granted_by ON TABLE user_role TYPE record<user>;
DEFINE FIELD OVERWRITE granted_at ON TABLE user_role TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE expires_at ON TABLE user_role TYPE option<datetime>;
DEFINE FIELD OVERWRITE is_active ON TABLE user_role TYPE bool DEFAULT true;
