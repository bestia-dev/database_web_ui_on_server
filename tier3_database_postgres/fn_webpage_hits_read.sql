create or replace function public.webpage_hits_read(
_id int)
returns table(id integer, webpage text, hit_count integer) 
language 'plpgsql'
as $body$
declare
begin

return query 
select w.id, w.webpage, w.hit_count
from webpage_hits w
where w.id=_id;

end; 
$body$;
