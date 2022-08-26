CREATE OR REPLACE FUNCTION public.webpage_update(
_webpage_id int,
_webpage text,
_counter integer)
RETURNS TABLE(id integer, webpage text, count integer) 
LANGUAGE 'plpgsql'
AS $BODY$
declare
begin
update webpage as W
set webpage = _webpage
where W.id = _webpage_id;
update hit_counter as H
set count = _counter
where H.webpage_id=_webpage_id;
return query 
select W.id, W.webpage, H.count
from webpage W
join hit_counter H on H.webpage_id = W.id
where W.id=_webpage_id;
end; 
$BODY$;
