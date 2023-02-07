CREATE TABLE services(
    id SERIAL PRIMARY KEY,
    service_owner INT NOT NULL,
    service_user INT NOT NULL,
    advert_id INT NOT NULL,
    CONSTRAINT fk_service_owner
        FOREIGN KEY(service_owner) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE CASCADE,
    CONSTRAINT fk_service_user
        FOREIGN KEY(service_user) REFERENCES users(id) ON DELETE NO ACTION ON UPDATE CASCADE,
    CONSTRAINT fk_advert_id
        FOREIGN KEY(advert_id) REFERENCES adverts(id) ON DELETE NO ACTION ON UPDATE CASCADE,
    status INT NOT NULL
);