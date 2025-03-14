-- This file should undo anything in `up.sql`
ALTER TABLE company_snapshot
DROP CONSTRAINT fk_company_snapshot_company;

ALTER TABLE subscription
DROP CONSTRAINT fk_subscription_company;

ALTER TABLE notable_change
DROP CONSTRAINT fk_notable_change_company;