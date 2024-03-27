# Use the official Nginx image as the base image
FROM nginx:latest

WORKDIR /etc/nginx

ADD . ./

WORKDIR /etc/nginx/infra/docker/nginx

# Remove the default Nginx configuration file
RUN rm /etc/nginx/conf.d/default.conf

# copy the logs dir password into the /etc/nginx dir
COPY .htpasswd /etc/nginx/.htpasswd
# Copy the SSL certificate and private key to the container
#zoomate.app:
COPY cert-zoomate.app.pem /etc/nginx/cert-zoomate.app.pem
COPY key-zoomate.app.pem /etc/nginx/key-zoomate.app.pem
#api.hoopoe.zoomate.app:
COPY cert-api.hoopoe.zoomate.app.pem /etc/nginx/cert-api.hoopoe.zoomate.app.pem
COPY key-api.hoopoe.zoomate.app.pem /etc/nginx/key-api.hoopoe.zoomate.app.pem
#adminer.zoomate.app:
COPY cert-adminer.zoomate.app.pem /etc/nginx/cert-adminer.zoomate.app.pem
COPY key-adminer.zoomate.app.pem /etc/nginx/key-adminer.zoomate.app.pem
#portainer.zoomate.app:
COPY cert-portainer.zoomate.app.pem /etc/nginx/cert-portainer.zoomate.app.pem
COPY key-portainer.zoomate.app.pem /etc/nginx/key-portainer.zoomate.app.pem
#grafana.zoomate.app:
COPY cert-grafana.zoomate.app.pem /etc/nginx/cert-grafana.zoomate.app.pem
COPY key-grafana.zoomate.app.pem /etc/nginx/key-grafana.zoomate.app.pem

# COPY /etc/letsencrypt/live/deadcat.ir/fullchain.pem /etc/nginx/cert.pem
# COPY /etc/letsencrypt/live/deadcat.ir/privkey.pem /etc/nginx/key.pem
COPY options-ssl-nginx.conf /etc/nginx/options-ssl-nginx.conf
COPY ssl-dhparams.pem /etc/nginx/ssl-dhparams.pem

# Expose port 80 for HTTP traffic and port 443 for HTTPS traffic
EXPOSE 80
EXPOSE 443

COPY nginx.conf /etc/nginx/nginx.conf
COPY zoomate.app.conf /etc/nginx/zoomate.app.conf
COPY api.hoopoe.zoomate.app.conf /etc/nginx/api.hoopoe.zoomate.app.conf
COPY adminer.zoomate.app.conf /etc/nginx/adminer.zoomate.app.conf
COPY portainer.zoomate.app.conf /etc/nginx/portainer.zoomate.app.conf
COPY grafana.zoomate.app.conf /etc/nginx/grafana.zoomate.app.conf

# Start Nginx in the foreground when the Docker container starts
CMD ["nginx", "-g", "daemon off;"]