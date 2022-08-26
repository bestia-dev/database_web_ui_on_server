CREATE OR REPLACE FUNCTION public.webpage_read(
_webpage_id int)
RETURNS TABLE(id integer, webpage text, count integer) 
LANGUAGE 'plpgsql'
AS $BODY$
declare
begin
return query 
select W.id, W.webpage, H.count
from webpage W
join hit_counter H on H.webpage_id = W.id
where W.id=_webpage_id;
end; 
$BODY$;
