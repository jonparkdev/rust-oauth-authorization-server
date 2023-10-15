-- Add migration script here
CREATE TABLE clients(
   id uuid NOT NULL,
   PRIMARY KEY (id),
   client_id TEXT NOT NULL UNIQUE,
   client_secret TEXT NOT NULL,
   redirect_uris TEXT [] NOT NULL CHECK (array_length(redirect_uris, 1) > 0),
   created_at timestamptz NOT NULL
);