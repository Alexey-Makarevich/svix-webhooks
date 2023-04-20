
# First run:
- start redis cluster ```docker compose -f docker-compose-mysql.yml up -d redis-cluster```
- start MySql or use external
- start only 1 instance of svix ```docker compose -f docker-compose-mysql.yml up -d backend```
- start 2 instance of svix ```docker compose -f docker-compose-mysql.yml up -d backend2```



### ACLs for redis-cluster
docker exec svix-mysql-redis-cluster-redis-cluster-1 bash -c "echo 'acl setuser svix on >svixpass ~svix:dev:* +@all' | redis-cli; echo 'ACL LIST' | redis-cli "
docker exec svix-mysql-redis-cluster-redis-cluster-node-0-1 bash -c "echo 'acl setuser svix on >svixpass ~svix:dev:* +@all' | redis-cli; echo 'ACL LIST' | redis-cli "
docker exec svix-mysql-redis-cluster-redis-cluster-node-1-1 bash -c "echo 'acl setuser svix on >svixpass ~svix:dev:* +@all' | redis-cli; echo 'ACL LIST' | redis-cli "
docker exec svix-mysql-redis-cluster-redis-cluster-node-2-1 bash -c "echo 'acl setuser svix on >svixpass ~svix:dev:* +@all' | redis-cli; echo 'ACL LIST' | redis-cli "
docker exec svix-mysql-redis-cluster-redis-cluster-node-3-1 bash -c "echo 'acl setuser svix on >svixpass ~svix:dev:* +@all' | redis-cli; echo 'ACL LIST' | redis-cli "
docker exec svix-mysql-redis-cluster-redis-cluster-node-4-1 bash -c "echo 'acl setuser svix on >svixpass ~svix:dev:* +@all' | redis-cli; echo 'ACL LIST' | redis-cli "
