CREATE TABLE stores (
    id UUID NOT NULL, -- e.g. 2f973f1d-19b7-4347-8a73-0dd43446927b

    name VARCHAR(64) NOT NULL, -- e.g. Foobar
    description VARCHAR(64) NOT NULL, -- e.g. Foobar

    PRIMARY KEY (id)
)