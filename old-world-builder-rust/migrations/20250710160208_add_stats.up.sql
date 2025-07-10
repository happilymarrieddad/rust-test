CREATE TABLE IF NOT EXISTS games(
    id              BIGSERIAL PRIMARY KEY NOT NULL
    ,name           CITEXT NOT NULL
    ,created_at     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
    ,updated_at     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
    ,active         BOOLEAN NOT NULL DEFAULT TRUE
    ,deleted_at     TIMESTAMP WITH TIME ZONE
    ,UNIQUE(name)
);

INSERT INTO games (id,name,active) VALUES (1,'Warhammer Old World 1st Edition',TRUE);

CREATE TABLE IF NOT EXISTS stats(
    id              BIGSERIAL PRIMARY KEY NOT NULL
    ,game_id        BIGINT REFERENCES games(id)
    ,display        CITEXT NOT NULL
    ,position       INTEGER
    ,created_at     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
    ,updated_at     TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL
    ,active         BOOLEAN NOT NULL DEFAULT TRUE
    ,deleted_at     TIMESTAMP WITH TIME ZONE
    ,unique(game_id,display,position)
);

INSERT INTO stats (game_id,display,position,active) VALUES 
    (1,'M',1,TRUE)
    ,(1,'WS',2,TRUE)
    ,(1,'BS',3,TRUE)
    ,(1,'S',4,TRUE)
    ,(1,'T',5,TRUE)
    ,(1,'W',6,TRUE)
    ,(1,'I',7,TRUE)
    ,(1,'A',8,TRUE)
    ,(1,'Ld',9,TRUE);
