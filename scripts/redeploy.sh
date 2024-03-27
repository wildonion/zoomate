#!/bin/bash

cd ..

echo \t"------------------------------------------------------"\n
echo \t"  --[are you completed with .env.prod screct vars?]--"
echo \t"------------------------------------------------------"\n
read ENVCOMPLETED

if [[ $ENVCOMPLETED == "Y" || $ENVCOMPLETED == "y" ]]; then
    sudo rm .env && sudo mv .env.prod .env
    echo "[?] Enter Machine Id: "
    read MACHINE_ID
    echo MACHINE_ID=$MACHINE_ID >> .env
    echo "[?] Enter Node Id: "
    read NODE_ID
    echo NODE_ID=$NODE_ID >> .env

    echo "[?] Redis/Postgres Password: "
    read PSWD

    echo "[?] App Name: "
    read APPNAME

    sudo chmod 666 /var/run/docker.sock
    export SERVER_IP=$(hostname -I | awk '{print $1}')
    export PASSWORD=$PSWD
    export APP_NAME=$APPNAME

    echo "[?] Wanna Redeploy Infrastructure? "
    read REDPLOY_INFRASTRUCTURE

    if [[ $REDPLOY_INFRASTRUCTURE == "Y" || $REDPLOY_INFRASTRUCTURE == "y" ]]; then

        echo "> Redeploying Infrastructure Pipelines Only"
        echo "☕ Okay, sit back and drink your coffee :)"

        sudo docker stop graphana && sudo docker rm -f graphana
        sudo docker stop postgres && sudo docker rm -f postgres
        sudo docker stop adminer && sudo docker rm -f adminer
        sudo docker stop nginx && sudo docker rm -f nginx
        sudo docker stop redis && sudo docker rm -f redis
        sudo docker stop portainer && sudo docker rm -f portainer
        sudo docker stop spacetimedb && sudo docker rm -f spacetimedb

        sudo docker run -d --network gem --name spacetimedb --rm --pull always -p 7556:80 clockworklabs/spacetimedb start

        sudo docker run -d --network gem -p 7050:3000 --name=grafana --user "$(id -u)" --volume $(pwd)/infra/data:/var/lib/grafana grafana/grafana

        docker volume create portainer_data
        docker run -d \
        -p 8000:8000 \
        -p 9443:9443 \
        --name portainer \
        --restart=always \
        --volume /var/run/docker.sock:/var/run/docker.sock \
        --volume portainer_data:/data \
        portainer/portainer-ce:latest

        sudo docker run -d \
        -h redis \
        -e REDIS_PASSWORD=$PASSWORD \
        -v $(pwd)/infra/data/redis/:/data \
        -p 6379:6379 \
        --name redis \
        --network gem \
        --restart always \
        redis:latest /bin/sh -c 'redis-server --appendonly yes --requirepass ${REDIS_PASSWORD}'

        sudo docker run -d --network gem --name postgres --restart unless-stopped -p 5432:5432 -v $(pwd)/infra/data/postgres/:/var/lib/postgresql/data -e POSTGRES_PASSWORD=$PASSWORD -e POSTGRES_USER=postgres -e PGDATA=/var/lib/postgresql/data/pgdata postgres
        sudo docker run -d --link postgres --network gem --name adminer -p 7543:8080 adminer
        diesel setup && diesel migration run
        sqlant postgresql://postgres:$PASSWORD@localhost/zoomate > $(pwd)/infra/zoomate.uml
        java -jar $(pwd)/infra/plantuml.jar $(pwd)/infra/zoomate.uml

        jobs="jobs/*"
        for f in $jobs
        do
            crontab $f
        done  
        crontab -u root -l 

        sudo docker ps -a && sudo docker compose ps -a && sudo docker images
    
    else
        echo "> Redeploying Rust Services Only"\n

        sudo rm -r $(pwd)/target

        ANY_ZOOMATE_HOOPOE_CONTAINER_ID=$(docker container ls  | grep 'zoomate-hoopoe' | awk '{print $1}')
        ANY_ZOOMATE_NODE_CONTAINER_ID=$(docker container ls  | grep 'zoomate-node' | awk '{print $1}')

        sudo docker stop $ANY_ZOOMATE_HOOPOE_CONTAINER_ID && sudo docker rm -f $ANY_ZOOMATE_HOOPOE_CONTAINER_ID

        TIMESTAMP=$(date +%s)

        echo \t"--[make sure you 1. setup a subdomain for wehbook endpoint in DNS records 2. register the webhook endpoint in your stripe dashabord 3. setup the nginx config file for the endpoint with SSL points to this VPS]--"

        echo \t"generating spacetimehoopoedb wasm methdos..."
        sudo mkdir -p $(pwd)/hoopoe/spacetimedb/client/hoopoedb
        spacetime generate --lang rust --out-dir $(pwd)/hoopoe/spacetimedb/client/hoopoedb --project-path $(pwd)/core/hoopoedb
        
        sudo docker build -t zoomate-hoopoe-$TIMESTAMP -f $(pwd)/infra/docker/hoopoe/Dockerfile . --no-cache
        sudo docker run -d --restart unless-stopped --link postgres --network gem --name zoomate-hoopoe-$TIMESTAMP -p 2345:2344 -v $(pwd)/assets/:/app/assets -v $(pwd)/infra/logs/:/app/logs zoomate-hoopoe-$TIMESTAMP

        sudo docker build -t zoomate-node-$TIMESTAMP -f $(pwd)/infra/docker/hoopoe/Dockerfile . --no-cache
        sudo docker run -d --restart unless-stopped --link postgres --network gem --name zoomate-node-$TIMESTAMP -p 2735:2734 -v $(pwd)/assets/:/app/assets -v $(pwd)/infra/logs/:/app/logs zoomate-node-$TIMESTAMP
        
        echo \n"you can run ./renew.sh to make containers alive!"

    fi

else
    echo \t"run me again once you get done with filling .env.prod vars"
fi