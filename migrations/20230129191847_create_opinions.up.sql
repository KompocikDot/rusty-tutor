CREATE TABLE opinions(
    id SERIAL PRIMARY KEY,
    evaluated_person_id INT NOT NULL,
    opinion_creator_id INT NOT NULL,
    advert_id INT NOT NULL,
    CONSTRAINT fk_evaluated_person
        FOREIGN KEY(evaluated_person_id) REFERENCES users(id) ON DELETE CASCADE ON UPDATE CASCADE,
    CONSTRAINT fk_opinion_creator
        FOREIGN KEY(opinion_creator_id) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE CASCADE,
    CONSTRAINT fk_avert
        FOREIGN KEY(advert_id) REFERENCES adverts(id) ON DELETE NO ACTION ON UPDATE CASCADE,
    content VARCHAR NOT NULL,
    rating INT NOT NULL 
);