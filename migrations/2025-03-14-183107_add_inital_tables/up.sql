-- Your SQL goes here
CREATE TABLE "company"(
	"company_house_id" TEXT NOT NULL PRIMARY KEY,
	"first_monitored_at" TIMESTAMP NOT NULL
);

CREATE TABLE "company_snapshot"(
	"id" UUID NOT NULL PRIMARY KEY,
	"company_house_id" TEXT NOT NULL,
	"snapshot_data" JSONB NOT NULL
);

CREATE TABLE "subscription"(
	"id" UUID NOT NULL PRIMARY KEY,
	"company_house_id" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL
);

CREATE TABLE "notable_change"(
	"id" UUID NOT NULL PRIMARY KEY,
	"subscription_id" UUID NOT NULL,
	"field" TEXT NOT NULL
);

