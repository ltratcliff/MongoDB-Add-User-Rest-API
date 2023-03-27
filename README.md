# Mongo Signup - Account Creation via REST

## Dev:
 ### Startup a mongo instance
```shell
podman run --rm -it -p 27017:27017 docker.io/library/mongo
```

### Run the Mongo Signup Service
```shell
API_ENDPOINT=/addMongoUser MONGODB_URI=mongodb://localhost cargo run
```

> :exclamation: The API_ENDPOINT and MONGODB_URI ENV vars are required to run

### Test the account creation
Health Check endpoint
```shell
curl http://localhost:3000/health-check
```
Create User endpoint
```shell
curl --header "Content-Type: application/json" --request POST --data '{"firstName":"Tom","lastName":"Ratcliff","orgName":"CDAO","dbName":"tomsDB","email":"tom2@email.com"}'  http://localhost:3000/addMongoUser
```

---

## Prod:
We host our APIs behind NGINX. There are 2 steps
1. Create Nginx config
2. Create Red Hat Systemd Service


### Nginx Sample Config
in /etc/nginx/sites-available:
1. Create config - mongoUserApi.conf
```nginx
location /api/addMongoUser {
	    proxy_pass http://127.0.0.1:3000/addMongoUser/;
	    proxy_set_header  Host $host;
	    proxy_set_header  X-Real-IP $remote_addr;
	    proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
    }
```
2. Link the config
```shell
ln -s /etc/nginx/sites-enabled/mongoUserApi.conf /etc/nginx/sites-available/mongoUserApi.conf
```
3. Reload Nginx config
```shell
nginx -t
nginx -s reload
```

### Red Hat Systemd Service
In /etc/systemd/system:
1. Create conf - mongoUserApi.service
```ini
[Unit]
Description=REST API to add MongoDB User

[Service]
Type=simple
WorkingDirectory=/opt/scripts/rust/bin/app
EnvironmentFile=/opt/scripts/rust/bin/app/.env
ExecStart=/opt/scripts/rust/bin/app
Restart=always
RestartSec=15
TimeoutStopSec=20

[Install]
WantedBy=multi-user.target
```
4. Start/check service
```shell
systemctl start mongoUserApi && systemctl status mongoUserApi
```
