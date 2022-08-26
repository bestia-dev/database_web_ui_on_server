CREATE OR REPLACE VIEW public.webpage_view01
AS
SELECT w.id,
w.webpage,
h.count
FROM webpage w
JOIN hit_counter h ON h.webpage_id = w.id
WHERE w.id = h.webpage_id
order by w.webpage;
