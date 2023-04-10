
# First run:
- start redis cluster ```docker compose -f docker-compose-mysql.yml up -d redis-cluster```
- start MySql or use external
- start only 1 instance of svix ```docker compose -f docker-compose-mysql.yml up -d backend```
- start 2 instance of svix ```docker compose -f docker-compose-mysql.yml up -d backend2```

