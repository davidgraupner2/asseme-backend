DEFINE TABLE OVERWRITE billing SCHEMAFULL
    PERMISSIONS 
        FOR select WHERE fn::has_permission(<string>$auth.id, 'billing.view_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'billing.view_own', <string>tenant) OR
                        fn::has_permission(<string>$auth.id, 'billing.view_customers', <string>tenant)
        FOR create WHERE fn::has_permission(<string>$auth.id, 'billing.create_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'billing.create_customers', <string>tenant)
        FOR update WHERE fn::has_permission(<string>$auth.id, 'billing.update_all', NONE) OR
                        fn::has_permission(<string>$auth.id, 'billing.update_own', <string>tenant) OR
                        fn::has_permission(<string>$auth.id, 'billing.update_customers', <string>tenant)
        FOR delete WHERE fn::has_permission(<string>$auth.id, 'billing.delete_all', NONE);
DEFINE FIELD OVERWRITE tenant ON TABLE billing TYPE record<tenant>;
DEFINE FIELD OVERWRITE responsible_party ON TABLE billing TYPE record<tenant>; -- Who actually pays
DEFINE FIELD OVERWRITE billing_period_start ON TABLE billing TYPE datetime;
DEFINE FIELD OVERWRITE billing_period_end ON TABLE billing TYPE datetime;
DEFINE FIELD OVERWRITE amount ON TABLE billing TYPE number ASSERT $value >= 0;
DEFINE FIELD OVERWRITE currency ON TABLE billing TYPE string DEFAULT 'USD';
DEFINE FIELD OVERWRITE status ON TABLE billing TYPE string ASSERT $value IN ['draft', 'pending', 'paid', 'overdue', 'cancelled'] DEFAULT 'draft';
DEFINE FIELD OVERWRITE invoice_number ON TABLE billing TYPE string;
DEFINE FIELD OVERWRITE msp_commission ON TABLE billing TYPE option<number>;
DEFINE FIELD OVERWRITE payment_due_date ON TABLE billing TYPE datetime;
DEFINE FIELD OVERWRITE created_at ON TABLE billing TYPE datetime DEFAULT time::now();
DEFINE FIELD OVERWRITE updated_at ON TABLE billing TYPE datetime VALUE time::now();
