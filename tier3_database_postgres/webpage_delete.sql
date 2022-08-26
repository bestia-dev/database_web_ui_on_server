CREATE OR REPLACE FUNCTION public.webpage_delete(
_webpage_id integer)
RETURNS void
LANGUAGE 'plpgsql'
AS $BODY$
declare
begin
delete from hit_counter H
where H.webpage_id = _webpage_id;
delete from webpage W
where W.id = _webpage_id;
end; 
$BODY$;