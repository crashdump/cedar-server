CREATE TABLE policies (
     store_id UUID NOT NULL,

     id UUID NOT NULL,
     description VARCHAR(64) NOT NULL,
     statement TEXT NOT NULL,

     PRIMARY KEY (store_id, id)
)