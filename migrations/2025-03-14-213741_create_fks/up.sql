-- Your SQL goes here
ALTER TABLE company_snapshot
ADD CONSTRAINT fk_company_snapshot_company
FOREIGN KEY (company_house_id)
REFERENCES company(company_house_id);

ALTER TABLE subscription
ADD CONSTRAINT fk_subscription_company
FOREIGN KEY (company_house_id)
REFERENCES company(company_house_id);

ALTER TABLE notable_change
ADD CONSTRAINT fk_notable_change_company
FOREIGN KEY (subscription_id)
REFERENCES subscription(id);