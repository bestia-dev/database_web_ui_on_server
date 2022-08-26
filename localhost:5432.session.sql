drop function 

SELECT oid::regprocedure
FROM pg_proc
WHERE proname = 'webpage_read'

