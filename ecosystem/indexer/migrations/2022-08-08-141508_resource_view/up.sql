-- Your SQL goes here
select from write_set_changes as r
inner join
(select address, hash, max(version) as version 
from write_set_changes 
where address='0xc435326568cf2f8ceaf49fc58bd6d696af9266829a8aa90c8236e9b127904f8' group by (address,hash)) as r
on r.;