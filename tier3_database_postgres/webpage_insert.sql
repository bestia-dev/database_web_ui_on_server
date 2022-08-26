CREATE OR REPLACE FUNCTION public.webpage_insert(
_webpage text,
_counter integer)
RETURNS TABLE(id integer, webpage text, count integer) 
LANGUAGE 'plpgsql'
AS $BODY$
declare
_webpage_id int := random_between(1,1000000000);
begin
INSERT INTO webpage ( "id", webpage)
VALUES (_webpage_id, _webpage);
insert into hit_counter(webpage_id, "count")
values(_webpage_id,_counter);
return query 
select W.id, W.webpage, H.count
from webpage W
join hit_counter H on H.webpage_id = W.id
where W.id=_webpage_id;
end; 
$BODY$;
