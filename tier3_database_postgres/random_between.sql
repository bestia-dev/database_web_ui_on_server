CREATE OR REPLACE FUNCTION public.random_between(
low integer,
high integer)
RETURNS integer
LANGUAGE 'plpgsql'
STRICT
AS $BODY$
BEGIN
RETURN floor(random()* (high-low + 1) + low);
END;
$BODY$;
