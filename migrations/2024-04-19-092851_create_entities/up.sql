CREATE TABLE entities (
    store_id UUID NOT NULL, -- e.g. 2f973f1d-19b7-4347-8a73-0dd43446927b

    id VARCHAR(64) NOT NULL, -- e.g. file1.png
    type VARCHAR(64) NOT NULL, -- e.g. Photo

    PRIMARY KEY (store_id, id)
)