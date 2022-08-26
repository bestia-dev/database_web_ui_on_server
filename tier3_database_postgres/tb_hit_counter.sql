CREATE TABLE IF NOT EXISTS public.hit_counter
(
    id integer NOT NULL DEFAULT nextval('hit_counter_id_seq'::regclass),
    webpage_id integer NOT NULL,
    count integer NOT NULL,
    CONSTRAINT hit_counter_pkey PRIMARY KEY (id),
    CONSTRAINT webpage FOREIGN KEY (webpage_id)
        REFERENCES public.webpage (id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
        NOT VALID
)
