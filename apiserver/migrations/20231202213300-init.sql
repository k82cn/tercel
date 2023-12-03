CREATE TABLE IF NOT EXISTS objects (
    id              UUID        NOT NULL DEFAULT gen_random_uuid(),
    kind            VARCHAR(64) NOT NULL,
    namespace       VARCHAR(64) NOT NULL,
    name            VARCHAR(64) NOT NULL,
    version         INT         NOT NULL DEFAULT 0,
    spec            TEXT        NOT NULL,
    status          TEXT        NOT NULL,

    PRIMARY KEY(id),
    UNIQUE (kind, namespace, name)
);

CREATE INDEX objects_index ON objects (kind, namespace, name);