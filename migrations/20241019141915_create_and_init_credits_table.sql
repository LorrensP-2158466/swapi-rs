-- Add migration script here

DROP TABLE IF EXISTS credits;
CREATE TABLE credits(
    id SERIAL PRIMARY KEY,
    user_ID TEXT NOT NULL,
    amount BIGINT CHECK(amount >= 0)
);

INSERT INTO credits(user_id, amount)
VALUES
    ('1', 100), -- luke
    ('2', 100), -- vader
    ('3', 100), -- han solo
    ('4', 100), -- Leia
    ('5', 100) -- WilHuff
    -- droids can't have credits as they have not rights
    ON CONFLICT (id) DO NOTHING
;

