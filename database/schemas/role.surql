DEFINE TABLE OVERWRITE role SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE fn::has_permission(<string>$auth.id, 'role.view_all', NONE) OR fn::has_permission(<string>$auth.id, 'role.view_own_tenant', NONE)
        FOR create, update, delete WHERE fn::has_permission(<string>$auth.id, 'system.manage', NONE);
DEFINE FIELD OVERWRITE name ON TABLE role TYPE string ASSERT $value != NONE;
DEFINE FIELD OVERWRITE description ON TABLE role TYPE string;
DEFINE FIELD OVERWRITE level ON TABLE role TYPE number ASSERT $value >= 1 AND $value <= 5;
DEFINE FIELD OVERWRITE permissions ON TABLE role TYPE array<string>;
DEFINE FIELD OVERWRITE created_at ON TABLE role TYPE datetime DEFAULT time::now();
